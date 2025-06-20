pub(crate) use crate::general::component::Component;

/// The circuit structure, which contains all components and node information.
pub struct Circuit {
    /// List of circuit components.
    pub components: Vec<Component>,
    /// Total number of nodes in the circuit.
    pub nodes_count: usize,
    /// The index of the designated ground node.
    pub ground_node: usize
}

impl Circuit {
    pub fn new(nodes: usize, ground: usize) -> Self {
        Circuit {
            components: vec![],
            nodes_count: nodes,
            ground_node: ground,
        }
    }
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }
}