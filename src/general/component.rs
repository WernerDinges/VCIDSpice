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