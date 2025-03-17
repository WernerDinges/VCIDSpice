use vcid_spice::general::circuit::Circuit;
use vcid_spice::general::circuit::Component::{CurrentDc, Diode, Resistor};
use vcid_spice::simulation::op::simulate_op;

fn main() {
    // Create a circuit with 3 nodes and ground reference
    let n = 3;
    let mut circuit = Circuit::new(n, 0);

    // Add components
    circuit.add_component(CurrentDc { anode: 2, cathode: 0, current: 1.0 });
    circuit.add_component(Resistor { pin1: 0, pin2: 1, r: 5.0 });
    circuit.add_component(Diode { anode: 2, cathode: 1, i_s: 170e-9, n: 2.0 });
    circuit.add_component(Resistor { pin1: 0, pin2: 2, r: 5.0 });

    // Simulate operating point
    let voltages = simulate_op(&circuit, 0.05, 1e-3, None);
    
    println!("Node voltages:");
    for (i, voltage) in voltages.iter().enumerate() {
        println!("  Node {}: {:.4} V", i, voltage);
    }
} 