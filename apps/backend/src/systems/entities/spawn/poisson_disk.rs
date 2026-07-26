use crate::utils::etc::random_f32;
use bevy_tasks::ComputeTaskPool;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy)]
pub struct PoissonDiskSample {
    pub radius: f32,
    pub position: (f32, f32),
}

type Bounds = ((f32, f32), (f32, f32)); // ((min_x, min_y), (max_x, max_y))
type Cell = (i32, i32);

#[derive(Clone, Copy)]
struct Placed {
    pos: (f32, f32),
    radius: f32,
}

#[inline]
fn cell_coords(pos: (f32, f32), origin: (f32, f32), cell_size: f32) -> Cell {
    (
        ((pos.0 - origin.0) / cell_size).floor() as i32,
        ((pos.1 - origin.1) / cell_size).floor() as i32,
    )
}

fn is_valid(
    pos: (f32, f32),
    r: f32,
    tile_bounds: &Bounds,
    origin: (f32, f32),
    cell_size: f32,
    max_radius: f32,
    local_grid: &HashMap<Cell, Vec<usize>>,
    local_placed: &[Placed],
    global_grid: &HashMap<Cell, Vec<Placed>>,
) -> bool {
    if pos.0 - r < tile_bounds.0.0
        || pos.0 + r > tile_bounds.1.0
        || pos.1 - r < tile_bounds.0.1
        || pos.1 + r > tile_bounds.1.1
    {
        return false;
    }

    let (cx, cy) = cell_coords(pos, origin, cell_size);
    let search_radius = ((r + max_radius) / cell_size).ceil() as i32;

    for dx in -search_radius..=search_radius {
        for dy in -search_radius..=search_radius {
            let key = (cx + dx, cy + dy);

            if let Some(indices) = local_grid.get(&key) {
                for &li in indices {
                    let other = local_placed[li];
                    let ddx = other.pos.0 - pos.0;
                    let ddy = other.pos.1 - pos.1;
                    let min_dist = r + other.radius;
                    if ddx * ddx + ddy * ddy < min_dist * min_dist {
                        return false;
                    }
                }
            }

            if let Some(placed) = global_grid.get(&key) {
                for other in placed {
                    let ddx = other.pos.0 - pos.0;
                    let ddy = other.pos.1 - pos.1;
                    let min_dist = r + other.radius;
                    if ddx * ddx + ddy * ddy < min_dist * min_dist {
                        return false;
                    }
                }
            }
        }
    }
    true
}

/// Runs Bridson's algorithm confined to one tile. `global_grid` only holds
/// points finalized by *previously processed* colors, so reading it here
/// concurrently with other tiles of the same color is race-free.
fn sample_tile(
    queue: &[usize],
    radii: &[f32],
    tile_bounds: &Bounds,
    origin: (f32, f32),
    cell_size: f32,
    max_radius: f32,
    packing_efficiency: usize,
    global_grid: &HashMap<Cell, Vec<Placed>>,
) -> Vec<(usize, (f32, f32))> {
    if queue.is_empty() {
        return Vec::new();
    }

    let mut local_placed: Vec<Placed> = Vec::new();
    let mut local_grid: HashMap<Cell, Vec<usize>> = HashMap::new();
    let mut active_list: Vec<usize> = Vec::new();
    let mut remaining: VecDeque<usize> = queue.iter().copied().collect();
    let mut output: Vec<(usize, (f32, f32))> = Vec::with_capacity(queue.len());

    let first_gidx = remaining.pop_front().unwrap();
    let r0 = radii[first_gidx];
    let mut seeded = false;
    for _ in 0..packing_efficiency {
        let pos0 = (
            random_f32(tile_bounds.0.0 + r0, tile_bounds.1.0 - r0),
            random_f32(tile_bounds.0.1 + r0, tile_bounds.1.1 - r0),
        );
        if is_valid(
            pos0,
            r0,
            tile_bounds,
            origin,
            cell_size,
            max_radius,
            &local_grid,
            &local_placed,
            global_grid,
        ) {
            let li = local_placed.len();
            local_placed.push(Placed {
                pos: pos0,
                radius: r0,
            });
            local_grid
                .entry(cell_coords(pos0, origin, cell_size))
                .or_insert_with(Vec::new)
                .push(li);
            active_list.push(li);
            output.push((first_gidx, pos0));
            seeded = true;
            break;
        }
    }
    if !seeded {
        return output;
    }

    while !active_list.is_empty() && !remaining.is_empty() {
        let active_slot = fastrand::usize(0..active_list.len());
        let active_li = active_list[active_slot];
        let active_pos = local_placed[active_li].pos;
        let active_r = local_placed[active_li].radius;

        let candidate_gidx = *remaining.front().unwrap();
        let new_r = radii[candidate_gidx];

        let mut placed_ok = false;

        for _ in 0..packing_efficiency {
            let min_dist = active_r + new_r;
            let dist = random_f32(min_dist, 2.0 * min_dist);
            let angle = random_f32(0.0, std::f32::consts::TAU);

            let candidate_pos = (
                active_pos.0 + dist * angle.cos(),
                active_pos.1 + dist * angle.sin(),
            );

            if is_valid(
                candidate_pos,
                new_r,
                tile_bounds,
                origin,
                cell_size,
                max_radius,
                &local_grid,
                &local_placed,
                global_grid,
            ) {
                let li = local_placed.len();
                local_placed.push(Placed {
                    pos: candidate_pos,
                    radius: new_r,
                });
                local_grid
                    .entry(cell_coords(candidate_pos, origin, cell_size))
                    .or_insert_with(Vec::new)
                    .push(li);
                active_list.push(li);
                output.push((candidate_gidx, candidate_pos));
                remaining.pop_front();
                placed_ok = true;
                break;
            }
        }

        if !placed_ok {
            active_list.swap_remove(active_slot);
        }
    }

    output
}

/// Distributes disks of the given `radii` inside `bounds` using a tiled,
/// checkerboard-parallel variant of Bridson's Poisson-disk algorithm.
/// Parallelism runs on Bevy's own `ComputeTaskPool` — no extra crates.
pub fn poisson_disk_sampling(
    radii: &[f32],
    bounds: &Bounds,
    packing_efficiency: usize,
) -> Vec<PoissonDiskSample> {
    if radii.is_empty() {
        return Vec::new();
    }

    let pool = ComputeTaskPool::get();

    let max_radius = radii.iter().copied().fold(0.0_f32, f32::max);
    let min_radius = radii.iter().copied().fold(f32::MAX, f32::min);
    let cell_size = (min_radius.max(0.001)) * std::f32::consts::FRAC_1_SQRT_2;

    let world_w = bounds.1.0 - bounds.0.0;
    let world_h = bounds.1.1 - bounds.0.1;

    let n_threads = bevy_tasks::available_parallelism().max(1);
    let target_tiles = (n_threads * 8).max(4) as f32;
    let approx_tiles_per_axis = target_tiles.sqrt().ceil().max(1.0);
    let tile_size = (2.0 * max_radius)
        .max(cell_size * 4.0)
        .max(world_w.max(world_h) / approx_tiles_per_axis);

    let tiles_x = (world_w / tile_size).ceil().max(1.0) as i32;
    let tiles_y = (world_h / tile_size).ceil().max(1.0) as i32;
    let num_tiles = (tiles_x * tiles_y) as usize;

    let mut order: Vec<usize> = (0..radii.len()).collect();
    order.sort_by(|&a, &b| radii[b].partial_cmp(&radii[a]).unwrap());

    let mut tile_queues: Vec<Vec<usize>> = vec![Vec::new(); num_tiles];
    for (i, &idx) in order.iter().enumerate() {
        tile_queues[i % num_tiles].push(idx);
    }

    let tile_bounds = |tx: i32, ty: i32| -> Bounds {
        let min_x = bounds.0.0 + tx as f32 * tile_size;
        let min_y = bounds.0.1 + ty as f32 * tile_size;
        (
            (min_x, min_y),
            (
                (min_x + tile_size).min(bounds.1.0),
                (min_y + tile_size).min(bounds.1.1),
            ),
        )
    };

    let mut global_grid: HashMap<Cell, Vec<Placed>> = HashMap::new();
    let mut samples: Vec<Option<PoissonDiskSample>> = vec![None; radii.len()];

    for &(color_x, color_y) in &[(0, 0), (1, 0), (0, 1), (1, 1)] {
        let tiles_in_color = (0..tiles_y)
            .flat_map(|ty| (0..tiles_x).map(move |tx| (tx, ty)))
            .filter(|&(tx, ty)| tx % 2 == color_x && ty % 2 == color_y)
            .map(|(tx, ty)| (tx, ty, (ty * tiles_x + tx) as usize))
            .collect::<Vec<_>>();

        if tiles_in_color.is_empty() {
            continue;
        }

        let n_chunks = n_threads.min(tiles_in_color.len()).max(1);
        let chunk_size = (tiles_in_color.len() + n_chunks - 1) / n_chunks;

        // Scoped, borrowing spawn — same shape as `thread::scope`, but runs
        // on Bevy's own pool instead of raw OS threads.
        let batch_results = pool
            .scope(|s| {
                for chunk in tiles_in_color.chunks(chunk_size) {
                    let global_grid_ref = &global_grid;
                    let tile_queues_ref = &tile_queues;
                    let radii_ref = radii;
                    let tile_bounds_ref = &tile_bounds;

                    s.spawn(async move {
                        let mut chunk_out = Vec::new();
                        for &(tx, ty, qidx) in chunk {
                            let tb = tile_bounds_ref(tx, ty);
                            chunk_out.push(sample_tile(
                                &tile_queues_ref[qidx],
                                radii_ref,
                                &tb,
                                bounds.0,
                                cell_size,
                                max_radius,
                                packing_efficiency,
                                global_grid_ref,
                            ));
                        }
                        chunk_out
                    });
                }
            })
            .into_iter()
            .flatten();

        // Sequential merge — the only place global_grid is mutated.
        for tile_result in batch_results {
            for (idx, pos) in tile_result {
                let key = cell_coords(pos, bounds.0, cell_size);
                global_grid
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(Placed {
                        pos,
                        radius: radii[idx],
                    });
                samples[idx] = Some(PoissonDiskSample {
                    radius: radii[idx],
                    position: pos,
                });
            }
        }
    }

    samples.into_iter().flatten().collect()
}
