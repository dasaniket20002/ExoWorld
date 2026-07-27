use bevy_ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct SlotAllocator {
    next: u32,
    free: Vec<u32>,
}
impl SlotAllocator {
    pub fn alloc(&mut self) -> u32 {
        self.free.pop().unwrap_or_else(|| {
            let s = self.next;
            self.next += 1;
            s
        })
    }
    pub fn free(&mut self, slot: u32) {
        self.free.push(slot);
    }
    pub fn capacity(&self) -> usize {
        self.next as usize
    }
}
