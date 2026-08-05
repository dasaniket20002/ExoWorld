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
    let start = Instant::now();

    query.iter().for_each(|(entity, position)| {
        let cell_id = grid.world_to_cell_id(position.0, position.1);
        let chunk_id = grid.world_to_chunk_id(position.0, position.1);

        let cell_slot = grid.insert_entity_at(entity, chunk_id, cell_id);

        let location = GridLocation {
            chunk_id,
            cell_id,
            cell_slot,
        };

        cmd.entity(entity).insert(location);
    });

    // {
    //     grid.dump_to_file("frames/0_grid.txt");
    // }

    println!(
        "[INFO] Entities inserted into Spatial Grid in {:.2} ms",
        start.elapsed().as_secs_f32() * 1000.0,
    );
}
