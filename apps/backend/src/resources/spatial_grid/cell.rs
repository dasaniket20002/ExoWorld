use bevy_ecs::entity::Entity;

#[derive(Debug)]
pub struct Cell {
    id: usize,
    entities: Vec<Entity>,
}

impl<'a> Cell {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            entities: Vec::new(),
        }
    }

    #[inline]
    pub fn entities(&self) -> &Vec<Entity> {
        &self.entities
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    #[inline]
    pub fn push_entity(&mut self, entity: Entity) -> usize {
        self.entities.push(entity);
        self.entities.len() - 1
    }

    /// Returns the entity that gets swapped or `None` if entities becomes empty
    #[inline]
    pub fn remove_entity_at(&mut self, idx: usize) -> Option<&Entity> {
        // println!("FIRST: {} : {:?}", idx, &self.0);
        self.entities.swap_remove(idx);
        // println!("SECOND: {} : {:?}", idx, &self.0);

        let len = self.entities.len();

        if len > 0 && idx < len {
            unsafe { Some(self.entities.get_unchecked(idx)) }
        } else {
            None
        }
    }
}
