use rustc_hash::FxHashSet;
use crate::kernel::node::NodeId;

fn edge_key(a: NodeId, b: NodeId) -> u64 {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    ((lo as u64) << 32) | hi as u64
}

pub struct SparseGraph {
    adj: Vec<Vec<NodeId>>,       // node -> neighbors
    edges: FxHashSet<u64>,       // canonical edge keys
    active: FxHashSet<NodeId>,   // nodes with at least one neighbor
    pub node_count: usize,
}

impl SparseGraph {
    pub fn new(node_count: usize) -> Self {
        Self {
            adj: vec![Vec::new(); node_count],
            edges: FxHashSet::default(),
            active: FxHashSet::default(),
            node_count,
        }
    }

    pub fn add_edge(&mut self, a: NodeId, b: NodeId) -> bool {
        if a == b || a as usize >= self.node_count || b as usize >= self.node_count {
            return false;
        }
        let key = edge_key(a, b);
        if self.edges.insert(key) {
            self.adj[a as usize].push(b);
            self.adj[b as usize].push(a);
            self.active.insert(a);
            self.active.insert(b);
            true
        } else {
            false
        }
    }

    pub fn remove_edge(&mut self, a: NodeId, b: NodeId) -> bool {
        let key = edge_key(a, b);
        if self.edges.remove(&key) {
            self.adj[a as usize].retain(|&x| x != b);
            self.adj[b as usize].retain(|&x| x != a);
            if self.adj[a as usize].is_empty() { self.active.remove(&a); }
            if self.adj[b as usize].is_empty() { self.active.remove(&b); }
            true
        } else {
            false
        }
    }

    pub fn neighbors(&self, node: NodeId) -> &[NodeId] {
        &self.adj[node as usize]
    }

    pub fn degree(&self, node: NodeId) -> usize {
        self.adj[node as usize].len()
    }

    pub fn has_edge(&self, a: NodeId, b: NodeId) -> bool {
        self.edges.contains(&edge_key(a, b))
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn active_nodes(&self) -> &FxHashSet<NodeId> {
        &self.active
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (NodeId, NodeId)> + '_ {
        self.edges.iter().map(|&key| {
            let a = (key >> 32) as NodeId;
            let b = (key & 0xFFFF_FFFF) as NodeId;
            (a, b)
        })
    }
}
