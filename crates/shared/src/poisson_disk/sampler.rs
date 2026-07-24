use crate::utils::rng::random_f32;
use std::collections::{HashMap, VecDeque};

pub struct PoissonDiskSample {
    pub radius: f32,
    pub position: (f32, f32),
}

/// Distributes `samples` inside a world `bounds`
/// (centered at origin) using a variable-radius Poisson-disk-like algorithm.
///
/// `k` is the number of attempts made around each active point before giving up on it
/// (30 is the standard value used in Bridson's algorithm).
pub fn poisson_disk_sampling(
    mut samples: Vec<PoissonDiskSample>,
    bounds: ((f32, f32), (f32, f32)), // ((min_x, min_y), (max_x, max_y))
    packing_efficiency: usize,
) -> Vec<PoissonDiskSample> {
    if samples.is_empty() {
        return samples;
    }

    // Sort indices by radius, descending — place bigger samples first (fewer failures).
    let mut order: Vec<usize> = (0..samples.len()).collect();
    order.sort_by(|&a, &b| samples[b].radius.partial_cmp(&samples[a].radius).unwrap());

    let max_radius = samples.iter().map(|e| e.radius).fold(0.0_f32, f32::max);
    let min_radius = samples.iter().map(|e| e.radius).fold(f32::MAX, f32::min);

    // Cell size based on the smallest radius guarantees we never "skip" a close neighbor.
    let cell_size = (min_radius.max(0.001)) * std::f32::consts::FRAC_1_SQRT_2;

    let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();

    let cell_coords = |pos: (f32, f32)| -> (i32, i32) {
        (
            ((pos.0 - bounds.0.0) / cell_size).floor() as i32,
            ((pos.1 - bounds.0.1) / cell_size).floor() as i32,
        )
    };

    // Checks whether a candidate (pos, r) overlaps any already-placed entity,
    // and whether it stays within world bounds. Uses squared distance.
    let is_valid = |pos: (f32, f32),
                    r: f32,
                    grid: &HashMap<(i32, i32), Vec<usize>>,
                    samples: &Vec<PoissonDiskSample>|
     -> bool {
        if pos.0 - r < bounds.0.0
            || pos.0 + r > bounds.1.0
            || pos.1 - r < bounds.0.1
            || pos.1 + r > bounds.1.1
        {
            return false;
        }

        let (cx, cy) = cell_coords(pos);
        let search_radius = ((r + max_radius) / cell_size).ceil() as i32;

        for dx in -search_radius..=search_radius {
            for dy in -search_radius..=search_radius {
                if let Some(indices) = grid.get(&(cx + dx, cy + dy)) {
                    for &idx in indices {
                        let other = &samples[idx];
                        let ddx = other.position.0 - pos.0;
                        let ddy = other.position.1 - pos.1;
                        let dist_sq = ddx * ddx + ddy * ddy;
                        let min_dist = r + other.radius;
                        if dist_sq < min_dist * min_dist {
                            return false;
                        }
                    }
                }
            }
        }
        true
    };

    let mut active_list: Vec<usize> = Vec::new();
    let mut remaining: VecDeque<usize> = order.into_iter().collect();

    // Place the very first (largest) entity at a random position.
    let first_idx = remaining.pop_front().unwrap();
    let r0 = samples[first_idx].radius;
    let pos0 = (
        random_f32(bounds.0.0 + r0, bounds.1.0 - r0),
        random_f32(bounds.0.1 + r0, bounds.1.1 - r0),
    );
    samples[first_idx].position = pos0;
    grid.entry(cell_coords(pos0))
        .or_insert_with(Vec::new)
        .push(first_idx);
    active_list.push(first_idx);

    while !active_list.is_empty() && !remaining.is_empty() {
        let active_slot = fastrand::usize(0..active_list.len());
        let active_idx = active_list[active_slot];
        let active_pos = samples[active_idx].position;
        let active_r = samples[active_idx].radius;

        let candidate_idx = *remaining.front().unwrap();
        let new_r = samples[candidate_idx].radius;

        let mut placed = false;

        for _ in 0..packing_efficiency {
            let min_dist = active_r + new_r;
            let dist = random_f32(min_dist, 2.0 * min_dist);
            let angle = random_f32(0.0, std::f32::consts::TAU);

            let candidate_pos = (
                active_pos.0 + dist * angle.cos(),
                active_pos.1 + dist * angle.sin(),
            );

            if is_valid(candidate_pos, new_r, &grid, &samples) {
                samples[candidate_idx].position = candidate_pos;
                grid.entry(cell_coords(candidate_pos))
                    .or_insert_with(Vec::new)
                    .push(candidate_idx);
                active_list.push(candidate_idx);
                remaining.pop_front();
                placed = true;
                break;
            }
        }

        if !placed {
            active_list.swap_remove(active_slot);
        }
    }

    // Any samples left unplaced (active list died out before everyone
    // could be placed) are dropped from the returned vector entirely.
    let mut leftover_indices: Vec<usize> = remaining.into_iter().collect();
    leftover_indices.sort_unstable_by(|a, b| b.cmp(a)); // descending order

    for idx in leftover_indices {
        samples.swap_remove(idx);
    }

    samples
}
