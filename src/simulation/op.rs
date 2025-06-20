use std::f64::consts::E;
use crate::general::circuit::{Circuit, Component};
use crate::general::role::{ConstantCharges, ExponentialContributions, LinearContributions};

/// Simulates the operating point of the circuit by iteratively updating node voltages.
///
/// # Parameters
/// - `circuit`: The circuit description.
/// - `t_vir`: A scaling factor.
/// - `tolerance`: The convergence threshold (maximum allowed voltage change).
/// - `initial_voltages`: Optional initial guess for node voltages (if `None`, start at zero).
///
/// # Returns
/// A vector of node voltages, normalized relative to the ground node.
///
/// # Overview
///
/// This function builds a list of “roles” for each node, each of which encapsulates the
/// contribution (current injection) of a connected element. The iterative loop:
/// 1. Computes the total virtual charge at each node.
/// 2. Computes a voltage update based on the difference between nodes’ charges.
/// 3. Adapts the damping factor based on whether the error is decreasing or increasing.
/// 4. Checks for convergence.
/// 5. Normalizes the node voltages relative to the ground node.
pub fn simulate_op(
    circuit: &Circuit,
    t_vir: f64,
    tolerance: f64,
    initial_voltages: Option<Vec<f64>>,
) -> Vec<f64> {
    let mut voltages = initial_voltages.unwrap_or_else(|| vec![0.0; circuit.nodes_count]);
    let mut charges = vec![0.0; circuit.nodes_count];

    // --- Preprocessing: collect component contributions ---
    let mut consts = ConstantCharges { currents: vec![], target_nodes: vec![] };
    let mut linears = LinearContributions { conductances: vec![], neighbors: vec![], target_nodes: vec![] };
    let mut exps = ExponentialContributions { i_s: vec![], n: vec![], anodes: vec![], cathodes: vec![], flips: vec![], target_nodes: vec![], };

    for component in &circuit.components { match component {

        Component::VoltageDc { .. } => { /* TODO(IMPLEMENT) */ }

        Component::CurrentDc { anode, cathode, current } => {
            let scaled = current * t_vir;
            consts.currents.push(scaled);
            consts.target_nodes.push(*anode);
            consts.currents.push(-scaled);
            consts.target_nodes.push(*cathode);
        }

        Component::Resistor { pin1, pin2, r } => {
            let g = t_vir / r;
            linears.conductances.push(g);
            linears.neighbors.push(*pin2);
            linears.target_nodes.push(*pin1);
            linears.conductances.push(g);
            linears.neighbors.push(*pin1);
            linears.target_nodes.push(*pin2);
        }

        Component::Diode { anode, cathode, i_s, n } => {
            let scaled_i_s = i_s * t_vir;
            exps.i_s.push(scaled_i_s);
            exps.n.push(*n);
            exps.anodes.push(*anode);
            exps.cathodes.push(*cathode);
            exps.flips.push(-1.0);
            exps.target_nodes.push(*anode);

            exps.i_s.push(scaled_i_s);
            exps.n.push(*n);
            exps.anodes.push(*anode);
            exps.cathodes.push(*cathode);
            exps.flips.push(1.0);
            exps.target_nodes.push(*cathode);
        }

    } }

    // --- Simulation parameters ---
    let max_iterations = 10_000;
    let mut iteration = 0;
    let mut damper = 1.0;
    let mut prev_voltages = voltages.clone();
    let mut prev_error = f64::INFINITY;

    // --- Simulation loop ---
    loop {
        charges.fill(0.0);

        // Compute charge contributions (q_vir)
        for i in 0..consts.currents.len() {
            charges[consts.target_nodes[i]] += consts.currents[i];
        }
        for i in 0..linears.conductances.len() {
            let t = linears.target_nodes[i];
            let n = linears.neighbors[i];
            charges[t] += linears.conductances[i] * (voltages[n] - voltages[t]);
        }
        for i in 0..exps.i_s.len() {
            let v_diff = (voltages[exps.anodes[i]] - voltages[exps.cathodes[i]]).clamp(-5.0, 5.0);
            let current = exps.flips[i] * exps.i_s[i] * (E.powf(v_diff / (exps.n[i] * 0.025852)) - 1.0);
            charges[exps.target_nodes[i]] += current;
        }

        // Compute voltage updates
        let mut delta_vs = vec![0.0; circuit.nodes_count];
        for i in 0..linears.target_nodes.len() {
            let t = linears.target_nodes[i];
            let n = linears.neighbors[i];
            delta_vs[t] += damper * (charges[t] - charges[n]);
        }
        for i in 0..exps.target_nodes.len() {
            let t = exps.target_nodes[i];
            let n = exps.anodes[i] ^ exps.cathodes[i] ^ t; // derive neighbor
            delta_vs[t] += damper * (charges[t] - charges[n]);
        }

        // Update voltages
        for i in 0..circuit.nodes_count {
            voltages[i] += delta_vs[i];
        }

        // Error metric
        let max_delta_v = delta_vs.iter().map(|v| v.abs()).fold(0.0, f64::max);

        // Adaptive damping
        if max_delta_v > prev_error {
            voltages.clone_from(&prev_voltages);
            damper *= 0.5;
        } else {
            prev_voltages.clone_from(&voltages);
            prev_error = max_delta_v;
            damper = (damper * 1.1).min(1.0);
        }

        iteration += 1;

        if max_delta_v < tolerance {
            println!("Converged in {} iterations with damper {:.3}", iteration, damper);
            break;
        }
        if iteration >= max_iterations {
            println!("Warning: Max iterations reached without convergence.");
            break;
        }
    }

    // --- Normalize to ground ---
    let gnd_v = voltages[circuit.ground_node];
    for v in &mut voltages {
        *v -= gnd_v;
    }

    voltages
}