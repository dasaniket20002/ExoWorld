use crate::{
    components::{grid_location::GridLocation, position::Position},
    resources::spatial_grid::grid::SpatialGrid,
};
use bevy_ecs::{
    entity::Entity,
    system::{Commands, Query, ResMut},
};
use std::time::Instant;

pub fn insert_to_grid(
    mut cmd: Commands,
    mut grid: ResMut<SpatialGrid>,
    query: Query<(Entity, &Position)>,
) {
    // for chunk in &grid.active {
    //     for cell in chunk {
    //         print!("c ");
    //     }
    //     println!();
    // }
    let start = Instant::now();

    query.iter().for_each(|(entity, position)| {
        let location = GridLocation {
            cell_id: grid.world_to_cell_id(position.0, position.1),
            chunk_id: grid.world_to_chunk_id(position.0, position.1),
        };

        grid.insert_entity_at(entity, location.chunk_id, location.cell_id);

        cmd.entity(entity).insert(location);
    });

    println!(
        "[INFO] Entities inserted into Spatial Grid in {:.2} ms",
        start.elapsed().as_secs_f32() * 1000.0,
    );
}
