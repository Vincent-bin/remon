#[derive(Debug, Default)]
struct NodeT {
    pub first_in: i32,
    pub first_out: i32,
    pub prev: i32,
    pub next: i32,
}

#[derive(Debug, Default)]
struct ArcT {
    pub target: i32,
    pub source: i32,
    pub prev_in: i32,
    pub prev_out: i32,
    pub next_in: i32,
    pub next_out: i32,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Node {
    pub id: i32,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Arc {
    pub id: i32,
}

#[derive(Debug, Default)]
pub struct ListDigraphBase {
    pub id: i32,
    nodes: Vec<NodeT>,
    arcs: Vec<ArcT>,
    first_node: i32,
    first_free_node: i32,
    first_free_arc: i32,
}

impl ListDigraphBase {
    pub fn new() -> ListDigraphBase {
        ListDigraphBase {
            id: 0,
            nodes: Vec::new(),
            arcs: Vec::new(),
            first_node: -1,
            first_free_node: -1,
            first_free_arc: -1,
        }
    }

    pub fn max_node_id(&self) -> i32 {
        (self.nodes.len() - 1) as i32
    }

    pub fn max_arc_id(&self) -> i32 {
        (self.arcs.len() - 1) as i32
    }

    pub fn source(&self, arc: &Arc) -> Node {
        Node {
            id: self.arcs[arc.id as usize].source,
        }
    }

    pub fn target(&self, arc: &Arc) -> Node {
        Node {
            id: self.arcs[arc.id as usize].target,
        }
    }

    pub fn to_first_node(&self, node: &mut Node) {
        node.id = self.first_node;
    }

    pub fn to_next_node(&self, node: &mut Node) {
        node.id = self.nodes[node.id as usize].next;
    }

    pub fn to_first_arc(&self, arc: &mut Arc) {
        let mut n = self.first_node;
        while n != -1 && self.nodes[n as usize].first_out == -1 {
            n = self.nodes[n as usize].next;
        }
        arc.id = if n == -1 {
            -1
        } else {
            self.nodes[n as usize].first_out
        };
    }

    pub fn to_next_arc(&self, arc: &mut Arc) {
        if self.arcs[arc.id as usize].next_out != -1 {
            arc.id = self.arcs[arc.id as usize].next_out;
        } else {
            let mut n = self.nodes[self.arcs[arc.id as usize].source as usize].next;
            while n != -1 && self.nodes[n as usize].first_out == -1 {
                n = self.nodes[n as usize].next;
            }
            arc.id = if n == -1 {
                -1
            } else {
                self.nodes[n as usize].first_out
            };
        }
    }

    pub fn first_out(&self, arc: &mut Arc, node: &Node) {
        arc.id = self.nodes[node.id as usize].first_out;
    }
    
    pub fn next_out(&self, arc: &mut Arc) {
        arc.id = self.arcs[arc.id as usize].next_out;
    }
    
    pub fn first_in(&self, arc: &mut Arc, node: &Node) {
        arc.id = self.nodes[node.id as usize].first_in;
    }
    
    pub fn next_in(&self, arc: &mut Arc) {
        arc.id = self.arcs[arc.id as usize].next_in;
    }

    pub fn node_id(node: &Node) -> i32 {
        node.id
    }

    pub fn arc_id(arc: &Arc) -> i32 {
        arc.id
    }
}
