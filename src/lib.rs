//! # VCID Spice
//!
//! A circuit simulation library for electrical circuits.
//!
//! This crate provides tools for defining and simulating electrical circuits,
//! with support for both linear and non-linear components.
//!
//! ## Example
//!
//! ```rust
//! use vcid_spice::general::circuit::Circuit;
//! use vcid_spice::general::circuit::Component::{CurrentDc, Diode, Resistor};
//! use vcid_spice::simulation::op::simulate_op;
//!
//! // Create a circuit with 3 nodes and ground reference
//! let mut circuit = Circuit::new(3, 0);
//!
//! // Add components
//! circuit.add_component(CurrentDc { anode: 2, cathode: 0, current: 1.0 });
//! circuit.add_component(Resistor { pin1: 0, pin2: 1, r: 5.0 });
//! circuit.add_component(Diode { anode: 2, cathode: 1, i_s: 170e-9, n: 2.0 });
//! circuit.add_component(Resistor { pin1: 0, pin2: 2, r: 5.0 });
//!
//! // Simulate operating point
//! let voltages = simulate_op(&circuit, 0.05, 1e-3, None);
//! ```

pub mod general;
pub mod simulation;

#[cfg(test)]
mod tests {
    use crate::general::circuit::Circuit;
    use crate::general::circuit::Component::{CurrentDc, Diode, Resistor};
    use crate::simulation::op::simulate_op;

    #[test]
    fn circuit_op_nonlinear() {
        let n = 3;
        let mut circuit = Circuit::new(n, 0);

        circuit.add_component(CurrentDc { anode: 2, cathode: 0, current: 1. });
        circuit.add_component(Resistor { pin1: 0, pin2: 1, r: 5. });
        circuit.add_component(Diode { anode: 2, cathode: 1, i_s: 170e-9, n: 2.0 });
        circuit.add_component(Resistor { pin1: 0, pin2: 2, r: 5. });

        use std::time::Instant;
        let now = Instant::now();

        let voltages = simulate_op(&circuit, 0.05, 1e-3, None);

        let elapsed = now.elapsed();
        println!("{:?}", voltages);
        print!("{:?}", elapsed);
    }
}
