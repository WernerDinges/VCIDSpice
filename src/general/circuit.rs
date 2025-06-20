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

/// Definitions for circuit components.
pub enum Component {
    /// Voltage sources are defined here but are not handled by the iterative solver.
    VoltageDc { anode: usize, cathode: usize, v: f64 },
    /// DC current sources: these inject a constant current into the nodes.
    CurrentDc { anode: usize, cathode: usize, current: f64 },
    /// Resistors.
    Resistor { pin1: usize, pin2: usize, r: f64 },
    /// Diodes with an exponential current–voltage characteristic.
    Diode { anode: usize, cathode: usize, i_s: f64, n: f64 }
}