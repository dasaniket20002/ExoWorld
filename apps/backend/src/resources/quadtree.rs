use bevy_ecs::resource::Resource;
use bevy_math::Vec2;
use std::collections::{HashMap, hash_map::Entry};
use uuid::Uuid;

type NodeIndex = u32;
type SlotIndex = u32;

const MAX_ENTITIES_PER_NODE: usize = 16;
const MERGE_THRESHOLD: usize = MAX_ENTITIES_PER_NODE / 2; // hysteresis to avoid split/merge thrashing
const MAX_DEPTH: u8 = 14;
const LOOSENESS: f32 = 1.5; // expand factor for the "still inside this node" fast path

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub cx: f32,
    pub cy: f32,
    pub hw: f32, // half-width
    pub hh: f32, // half-height
}

impl Bounds {
    #[inline(always)]
    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.cx - self.hw
            && p.x < self.cx + self.hw
            && p.y >= self.cy - self.hh
            && p.y < self.cy + self.hh
    }

    #[inline(always)]
    fn loose(&self) -> Bounds {
        Bounds {
            cx: self.cx,
            cy: self.cy,
            hw: self.hw * LOOSENESS,
            hh: self.hh * LOOSENESS,
        }
    }

    /// Squared-radius circle test — callers precompute `r2` once per query
    /// instead of every node paying for a redundant `r * r`.
    #[inline(always)]
    fn intersects_circle_sq(&self, cx: f32, cy: f32, r2: f32) -> bool {
        let dx = (cx - self.cx).abs() - self.hw;
        let dy = (cy - self.cy).abs() - self.hh;
        let ex = dx.max(0.0);
        let ey = dy.max(0.0);
        ex * ex + ey * ey <= r2
    }

    /// Clamp a point into this rectangle
    #[inline(always)]
    fn clamp_point(&self, p: Vec2) -> Vec2 {
        let eps_x = (self.hw * 4.0 * f32::EPSILON).max(f32::MIN_POSITIVE);
        let eps_y = (self.hh * 4.0 * f32::EPSILON).max(f32::MIN_POSITIVE);
        Vec2::new(
            p.x.clamp(self.cx - self.hw, self.cx + self.hw - eps_x),
            p.y.clamp(self.cy - self.hh, self.cy + self.hh - eps_y),
        )
    }

    fn child(&self, quadrant: usize) -> Option<Bounds> {
        let hw = self.hw * 0.5;
        let hh = self.hh * 0.5;
        let (ox, oy) = match quadrant {
            0 => (-hw, -hh),
            1 => (hw, -hh),
            2 => (-hw, hh),
            3 => (hw, hh),
            _ => return Option::None,
        };
        Option::Some(Bounds {
            cx: self.cx + ox,
            cy: self.cy + oy,
            hw,
            hh,
        })
    }

    #[inline(always)]
    fn quadrant_of(&self, p: Vec2) -> usize {
        match (p.x >= self.cx, p.y >= self.cy) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        }
    }
}

enum NodeState {
    Branch([NodeIndex; 4]), // contains the children
    Leaf(Vec<SlotIndex>),   // contains the entries
}

struct Node {
    bounds: Bounds,
    loose_bounds: Bounds,
    depth: u8,
    parent: Option<NodeIndex>, // None for root
    node_state: NodeState,
}

struct EntitySlot {
    id: Uuid,
    pos: Vec2,
    node: NodeIndex,
    index_in_node: u32,
}

#[derive(Resource)]
pub struct Quadtree {
    nodes: Vec<Node>,
    free_nodes: Vec<NodeIndex>,
    entities: Vec<EntitySlot>,
    id_to_slot: HashMap<Uuid, SlotIndex>,
}

impl Quadtree {
    pub fn new(root_bounds: (u32, u32)) -> Self {
        let bounds = Bounds {
            cx: 0.0,
            cy: 0.0,
            hw: root_bounds.0 as f32 / 2.0,
            hh: root_bounds.1 as f32 / 2.0,
        };
        let root = Node {
            bounds,
            loose_bounds: bounds.loose(),
            depth: 0,
            parent: Option::None,
            node_state: NodeState::Leaf(Vec::new()),
        };
        Self {
            nodes: vec![root],
            free_nodes: Vec::new(),
            entities: Vec::new(),
            id_to_slot: HashMap::default(),
        }
    }

    #[inline(always)]
    fn is_leaf(&self, n: NodeIndex) -> bool {
        match self.nodes[n as usize].node_state {
            NodeState::Leaf(_) => true,
            NodeState::Branch(_) => false,
        }
    }

    // ---------- Node allocation with free-list reuse ----------

    fn alloc_node(&mut self, bounds: Bounds, depth: u8, parent: NodeIndex) -> NodeIndex {
        let loose_bounds = bounds.loose();
        if let Some(idx) = self.free_nodes.pop() {
            let node = &mut self.nodes[idx as usize];
            node.bounds = bounds;
            node.loose_bounds = loose_bounds;
            node.depth = depth;
            node.parent = Option::Some(parent);
            // Freed nodes are always leaves: `maybe_merge` only recycles a
            // branch's children after confirming every one of them is a
            // leaf, so this is never a `Branch` here.
            debug_assert!(matches!(node.node_state, NodeState::Leaf(_)));
            if let NodeState::Leaf(entries) = &mut node.node_state {
                entries.clear();
            }
            idx
        } else {
            self.nodes.push(Node {
                bounds,
                loose_bounds,
                depth,
                parent: Option::Some(parent),
                node_state: NodeState::Leaf(Vec::new()),
            });
            (self.nodes.len() - 1) as NodeIndex
        }
    }

    // ---------- Public API ----------

    /// Fast path: if the entity hasn't left its (loosened) node bounds, this is O(1).
    /// Slow path: remove + reinsert from root, only when it actually crosses a boundary.
    ///
    /// Handles both the "entity already exists" and "entity is new" cases
    /// with a single hashmap probe via the `Entry` API.
    pub fn insert_or_update(&mut self, id: &Uuid, pos: &Vec2) {
        // let pos = self.nodes[0].bounds.clamp_point(*pos);

        match self.id_to_slot.entry(*id) {
            Entry::Occupied(occ) => {
                let slot = *occ.get();
                self.update_slot(slot, *pos);
            }
            Entry::Vacant(vac) => {
                let slot = self.entities.len() as SlotIndex;
                self.entities.push(EntitySlot {
                    id: *id,
                    pos: *pos,
                    node: 0,
                    index_in_node: 0,
                });
                vac.insert(slot);
                self.insert_into_tree(slot, *pos);
            }
        }
    }

    fn update_slot(&mut self, slot: SlotIndex, new_pos: Vec2) {
        let node_idx = self.entities[slot as usize].node;
        let loose_bounds = self.nodes[node_idx as usize].loose_bounds;

        if loose_bounds.contains(new_pos) {
            self.entities[slot as usize].pos = new_pos;
            return;
        }

        self.remove_from_node(slot);
        self.entities[slot as usize].pos = new_pos;
        self.insert_into_tree(slot, new_pos);
    }

    pub fn remove(&mut self, id: Uuid) {
        let slot = match self.id_to_slot.remove(&id) {
            Some(s) => s,
            None => return,
        };
        self.remove_from_node(slot);

        // O(1) removal from the dense entity array
        let last = self.entities.len() as u32 - 1;
        self.entities.swap_remove(slot as usize);
        if slot != last {
            let (moved_id, moved_node, moved_idx) = {
                let e = &self.entities[slot as usize];
                (e.id, e.node, e.index_in_node)
            };
            self.id_to_slot.insert(moved_id, slot);
            if let NodeState::Leaf(entries) = &mut self.nodes[moved_node as usize].node_state {
                entries[moved_idx as usize] = slot;
            }
        }
    }

    pub fn position(&self, id: Uuid) -> Option<Vec2> {
        self.id_to_slot
            .get(&id)
            .map(|&s| self.entities[s as usize].pos)
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Allocating convenience API.
    pub fn query_radius(&self, cx: f32, cy: f32, r: f32, out: &mut Vec<Uuid>) {
        out.clear();
        let r2 = r * r;
        self.query_radius_visit(0, cx, cy, r2, &mut |id, _p| out.push(id));
    }

    /// Zero-allocation query using a callback — preferred for hot loops.
    pub fn query_radius_each<F: FnMut(Uuid, Vec2)>(&self, cx: f32, cy: f32, r: f32, f: &mut F) {
        let r2 = r * r;
        self.query_radius_visit(0, cx, cy, r2, f);
    }

    // ---------- Internal tree mechanics ----------

    fn insert_into_tree(&mut self, slot: SlotIndex, pos: Vec2) {
        let mut node_idx: NodeIndex = 0;
        loop {
            let bounds = self.nodes[node_idx as usize].bounds;

            match &mut self.nodes[node_idx as usize].node_state {
                NodeState::Leaf(entries) => {
                    let idx_in_node = entries.len() as u32;
                    entries.push(slot);
                    let len = entries.len();
                    self.entities[slot as usize].node = node_idx;
                    self.entities[slot as usize].index_in_node = idx_in_node;

                    if len > MAX_ENTITIES_PER_NODE {
                        // Only fetch `depth` when we actually might split.
                        let depth = self.nodes[node_idx as usize].depth;
                        if depth < MAX_DEPTH {
                            self.split(node_idx);
                        }
                    }
                    return;
                }
                NodeState::Branch(children) => {
                    let q = bounds.quadrant_of(pos);
                    node_idx = children[q];
                }
            }
        }
    }

    fn split(&mut self, node_idx: NodeIndex) {
        let (bounds, depth, entries) = {
            let node = &mut self.nodes[node_idx as usize];

            let NodeState::Leaf(entries) = &mut node.node_state else {
                return;
            };

            (node.bounds, node.depth, std::mem::take(entries))
        };

        let mut child_indices = [0u32; 4];
        for q in 0..4 {
            let child_bounds = bounds.child(q);
            if let Some(bounds) = child_bounds {
                child_indices[q] = self.alloc_node(bounds, depth + 1, node_idx);
            }
        }

        self.nodes[node_idx as usize].node_state = NodeState::Branch(child_indices);

        for slot in entries {
            let pos = self.entities[slot as usize].pos;
            let q = bounds.quadrant_of(pos);
            let child_idx = child_indices[q];

            if let NodeState::Leaf(entries) = &mut self.nodes[child_idx as usize].node_state {
                let idx_in_node = entries.len() as u32;
                entries.push(slot);

                self.entities[slot as usize].index_in_node = idx_in_node;
            }
            self.entities[slot as usize].node = child_idx;
        }
    }

    fn remove_from_node(&mut self, slot: SlotIndex) {
        let (node_idx, idx_in_node) = {
            let e = &self.entities[slot as usize];
            (e.node, e.index_in_node)
        };

        if let NodeState::Leaf(entries) = &mut self.nodes[node_idx as usize].node_state {
            entries.swap_remove(idx_in_node as usize);
            if (idx_in_node as usize) < entries.len() {
                let moved_slot = entries[idx_in_node as usize];
                self.entities[moved_slot as usize].index_in_node = idx_in_node;
            }
            self.maybe_merge(node_idx);
        }
    }

    /// Merge four sibling leaves back into their parent once they're sparse enough,
    /// recycling the freed node indices for future splits.
    fn maybe_merge(&mut self, node_idx: NodeIndex) {
        let parent_idx = match self.nodes[node_idx as usize].parent {
            Some(parent_idx) => parent_idx,
            None => return,
        };

        let children = match &self.nodes[parent_idx as usize].node_state {
            NodeState::Branch(children) => *children,
            NodeState::Leaf(_) => return,
        };

        for &c in &children {
            if !self.is_leaf(c) {
                return;
            }
        }

        let total: usize = children
            .iter()
            .map(|&c| match &self.nodes[c as usize].node_state {
                NodeState::Leaf(entries) => entries.len(),
                NodeState::Branch(_) => 0,
            })
            .sum();
        if total > MERGE_THRESHOLD {
            return;
        }

        let mut collected = Vec::with_capacity(total);
        for &c in &children {
            if let NodeState::Leaf(entries) = &mut self.nodes[c as usize].node_state {
                collected.append(entries);
            }
            self.free_nodes.push(c);
        }

        for (i, &slot) in collected.iter().enumerate() {
            self.entities[slot as usize].node = parent_idx;
            self.entities[slot as usize].index_in_node = i as u32;
        }
        self.nodes[parent_idx as usize].node_state = NodeState::Leaf(collected);

        self.maybe_merge(parent_idx);
    }

    fn query_radius_visit<F: FnMut(Uuid, Vec2)>(
        &self,
        node_idx: NodeIndex,
        cx: f32,
        cy: f32,
        r2: f32,
        f: &mut F,
    ) {
        let node = &self.nodes[node_idx as usize];
        // Prune using `loose_bounds`, not the strict `bounds`. Entities can
        // drift up to `LOOSENESS`x their leaf's half-extent away without
        // triggering a tree relocation (see `update_slot`'s fast path), so
        // pruning on strict bounds can produce false negatives. A node's
        // own loose bounds are provably large enough to cover every
        // descendant leaf's maximum possible loose drift, regardless of
        // how many levels deep that leaf is, so this single check at every
        // level is both correct and sufficient.
        if !node.loose_bounds.intersects_circle_sq(cx, cy, r2) {
            return;
        }

        match &node.node_state {
            NodeState::Leaf(entries) => {
                for &slot in entries {
                    let e = &self.entities[slot as usize];
                    let dx = e.pos.x - cx;
                    let dy = e.pos.y - cy;
                    if dx * dx + dy * dy <= r2 {
                        f(e.id, e.pos);
                    }
                }
            }
            NodeState::Branch(children) => {
                for &child in children {
                    self.query_radius_visit(child, cx, cy, r2, f);
                }
            }
        }
    }
}
