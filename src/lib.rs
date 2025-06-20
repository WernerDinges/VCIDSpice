pub mod general;
pub mod simulation;

#[cfg(test)]
mod tests {
    use crate::general::circuit::{Circuit, Component};
    use crate::general::circuit::Component::{VoltageDc, Diode, Resistor, CurrentDc};
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
        let timer = Instant::now();

        let voltages = simulate_op(&circuit, 0.05, 1e-3, None);

        let elapsed = timer.elapsed();
        println!("{:?}", voltages);
        print!("{:?}", elapsed);
    }
}
