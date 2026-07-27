use bevy_tasks::ComputeTaskPool;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::utils::etc::par_write_ptr::ParallelWritePtr;

#[derive(Default)]
/// Generic compressed-sparse-row bucket table, built by counting sort.
pub struct CsrGrid {
    pub cell_start: Vec<u32>, // len = table_size + 1 (monotonic prefix sums)
    pub cell_items: Vec<u32>, // len = n_active, slot indices grouped by cell
}

impl CsrGrid {
    /// Parallel counting sort: groups `0..n` items into `table_size` buckets
    /// according to `key_of(i)`. O(n) work, fully parallel, no rebalancing.
    pub fn build_csr(n: usize, table_size: usize, key_of: impl Fn(usize) -> u32 + Sync) -> CsrGrid {
        let pool = ComputeTaskPool::get();
        let threads = pool.thread_num().max(1);
        let chunk = n.div_ceil(threads);

        let counts: Vec<AtomicU32> = (0..=table_size).map(|_| AtomicU32::new(0)).collect();

        // Phase 1 — parallel histogram
        pool.scope(|s| {
            for t in 0..threads {
                let (lo, hi) = (t * chunk, ((t + 1) * chunk).min(n));
                if lo >= hi {
                    continue;
                }
                let counts = &counts;
                let key_of = &key_of;
                s.spawn(async move {
                    for i in lo..hi {
                        counts[key_of(i) as usize].fetch_add(1, Ordering::Relaxed);
                    }
                });
            }
        });

        // Phase 2 — exclusive prefix sum (sequential, O(table_size), ~1-2ms at 20M buckets)
        let mut cell_start = vec![0u32; table_size + 1];
        let mut running = 0u32;
        for c in 0..table_size {
            cell_start[c] = running;
            running += counts[c].load(Ordering::Relaxed);
        }
        cell_start[table_size] = running;

        // Phase 3 — parallel scatter using atomic cursors seeded from cell_start
        let cursor: Vec<AtomicU32> = cell_start.iter().map(|&v| AtomicU32::new(v)).collect();
        let mut cell_items = vec![0u32; n];
        let out = ParallelWritePtr::new(cell_items.as_mut_ptr());

        pool.scope(|s| {
            for t in 0..threads {
                let (lo, hi) = (t * chunk, ((t + 1) * chunk).min(n));
                if lo >= hi {
                    continue;
                }
                let cursor = &cursor;
                let key_of = &key_of;
                s.spawn(async move {
                    let out = out; // <-- forces capture of the whole struct, not just `.0`
                    for i in lo..hi {
                        let dst =
                            cursor[key_of(i) as usize].fetch_add(1, Ordering::Relaxed) as usize;
                        unsafe {
                            out.write(dst, i as u32);
                        }
                    }
                });
            }
        });

        CsrGrid {
            cell_start,
            cell_items,
        }
    }
}
