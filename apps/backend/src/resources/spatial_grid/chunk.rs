use crate::{resources::spatial_grid::cell::Cell, utils::etc::sorted_vec::SortedVec};
use bevy_ecs::entity::{Entity, EntityHashMap};

#[derive(Debug)]
pub struct ChunkMutations {
    pub removals: SortedVec<(Entity, usize /*cell_id*/, usize /*slot*/)>,
    pub insertions: SortedVec<(Entity, usize /*cell_id*/)>,
}
impl ChunkMutations {
    fn new() -> Self {
        Self {
            removals: SortedVec::new(),
            insertions: SortedVec::new(),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.removals.clear();
        self.insertions.clear();
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub id: usize,
    pub cells: Vec<Cell>,
    pub mutations: ChunkMutations,
}

impl Chunk {
    pub fn with_cells(id: usize, cells: Vec<Cell>) -> Self {
        Self {
            id,
            cells,
            mutations: ChunkMutations::new(),
        }
    }

    #[inline]
    pub fn get_cell_mut(&mut self, cell_idx: usize) -> &mut Cell {
        unsafe { self.cells.get_unchecked_mut(cell_idx) }
    }

    #[inline]
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }

    pub fn queue_removal(&mut self, entity: Entity, cell_id: usize, cell_slot: usize) {
        self.mutations
            .removals
            .insert_sorted_by((entity, cell_id, cell_slot), |a, b| {
                a.1.cmp(&b.1).then_with(|| b.2.cmp(&a.2))
            });
    }

    pub fn queue_insertion(&mut self, entity: Entity, cell_id: usize) {
        self.mutations.insertions.push((entity, cell_id));
    }

    pub fn process_removals(&mut self, local_mutations: &mut EntityHashMap<(usize, usize, usize)>) {
        let (cells, removals) = (&mut self.cells, &self.mutations.removals);

        removals
            .iter()
            .for_each(|&(_, cell_idx, slot)| unsafe {
                // println!(
                //     "Removing {:?} from ({}, {}); chunk: {}, cell[{}]: {:?}",
                //     entity,
                //     cell_idx,
                //     slot,
                //     self.id,
                //     cell_idx,
                //     cells.get(cell_idx).unwrap().entities()
                // );

                let optional_swapped = cells.get_unchecked_mut(cell_idx).remove_entity_at(slot);

                if let Some(swapped) = optional_swapped {
                    local_mutations.insert(*swapped, (self.id, cell_idx, slot));
                }
            });
    }

    pub fn process_insertions(
        &mut self,
        local_mutations: &mut EntityHashMap<(usize, usize, usize)>,
    ) {
        let (cells, insertions) = (&mut self.cells, &self.mutations.insertions);

        insertions.iter().for_each(|&(entity, cell_idx)| unsafe {
            let slot = cells.get_unchecked_mut(cell_idx).push_entity(entity);
            local_mutations.insert(entity, (self.id, cell_idx, slot));
        });
    }
}
