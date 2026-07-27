use bevy_ecs::system::{Res, ResMut};
use bevy_tasks::ComputeTaskPool;

use crate::resources::spatial_hash_grid::{
    collision_pairs::CollisionPairs, spatial_grid::SpatialGrid,
};

pub fn detect_collisions(grid: Res<SpatialGrid>, mut out: ResMut<CollisionPairs>) {
    let pool = ComputeTaskPool::get();
    let n = grid.pos_x.len();
    let threads = pool.thread_num().max(1);
    let chunk = n.div_ceil(threads);
    let table_mask = (1u32 << grid.fine_table_bits) - 1;

    let results: Vec<Vec<(u32, u32)>> = pool.scope(|s| {
        for t in 0..threads {
            let (lo, hi) = (t * chunk, ((t + 1) * chunk).min(n));
            let grid = &grid;
            s.spawn(async move {
                let mut local = Vec::new();
                for i in lo..hi {
                    if !grid.alive[i] {
                        continue;
                    }
                    let cx = ((grid.pos_x[i] - grid.origin_x) / grid.fine_cell_size).floor() as i32;
                    let cy = ((grid.pos_y[i] - grid.origin_y) / grid.fine_cell_size).floor() as i32;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let hx = ((cx + dx) as u32).wrapping_mul(0x8da6b343);
                            let hy = ((cy + dy) as u32).wrapping_mul(0xd8163841);
                            let key = (hx ^ hy) & table_mask;
                            let s0 = grid.fine.cell_start[key as usize] as usize;
                            let s1 = grid.fine.cell_start[key as usize + 1] as usize;
                            for &j in &grid.fine.cell_items[s0..s1] {
                                if j <= i as u32 {
                                    continue;
                                } // canonical order, no double counting
                                let (ddx, ddy) = (
                                    grid.pos_x[j as usize] - grid.pos_x[i],
                                    grid.pos_y[j as usize] - grid.pos_y[i],
                                );
                                let rr = grid.radius[i] + grid.radius[j as usize];
                                if ddx * ddx + ddy * ddy <= rr * rr {
                                    local.push((i as u32, j));
                                }
                            }
                        }
                    }
                }
                local
            });
        }
    });

    out.0.clear();
    out.0.extend(results.into_iter().flatten());
}
