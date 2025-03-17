use crate::general::circuit::{Circuit, Component};
use crate::simulation::role::{Role};

/// Simulates the operating point of the circuit by iteratively updating node voltages.
///
/// # Parameters
/// - `circuit`: The circuit description.
/// - `t_vir`: A scaling factor (likely used to nondimensionalize currents/charges).
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
    // Initialize node voltages: if not provided, set all to zero.
    let mut voltages = initial_voltages.unwrap_or_else(|| vec![0.0; circuit.nodes_count]);

    // Allocate space for node charges.
    let mut charges = vec![0.0; circuit.nodes_count];

    // For each node, we keep a list of roles (i.e. contributions from components).
    let mut roles: Vec<Vec<Role>> = vec![Vec::new(); circuit.nodes_count];

    // --- Preprocessing: Build roles for each component ---
    for component in &circuit.components {
        match component {
            Component::VoltageDc { .. } => {
                // Voltage sources are not handled in the iterative update.
                // Their imposed voltages can be enforced after convergence.
            }
            Component::CurrentDc { anode, cathode, current } => {
                // Scale current injection by t_vir.
                let scaled_current = current * t_vir;
                roles[*anode].push(Role::ConstantCharge(scaled_current));
                roles[*cathode].push(Role::ConstantCharge(-scaled_current));
            }
            Component::Resistor { pin1, pin2, r } => {
                // The conductance is scaled by t_vir.
                let conductance = t_vir / r;
                roles[*pin1].push(Role::Linear {
                    conductance,
                    neighbor: *pin2,
                });
                roles[*pin2].push(Role::Linear {
                    conductance,
                    neighbor: *pin1,
                });
            }
            Component::Diode { anode, cathode, i_s, n } => {
                // Create two roles (one for each terminal) for the diode.
                roles[*anode].push(Role::Exponential {
                    i_s: i_s * t_vir,
                    n: *n,
                    neighbor: *cathode,
                    anode: *anode,
                    cathode: *cathode,
                    flip: -1.0,
                });
                roles[*cathode].push(Role::Exponential {
                    i_s: i_s * t_vir,
                    n: *n,
                    neighbor: *anode,
                    anode: *anode,
                    cathode: *cathode,
                    flip: 1.0,
                });
            }
        }
    }

    // --- Simulation loop parameters ---
    let max_iterations = 10_000;
    let mut iteration = 0;

    // Start with a damping factor of 1.0.
    let mut damper = 1.0;

    // For adaptive damping, store the previous voltage vector and error.
    let mut prev_voltages = voltages.clone();
    let mut prev_error = f64::INFINITY;

    // --- Main simulation loop ---
    loop {
        // 1. Compute the total charge at each node by summing contributions.
        for node in 0..circuit.nodes_count {
            let total_charge: f64 = roles[node]
                .iter()
                .map(|role| role.q_vir_impact(&voltages, node))
                .sum();
            charges[node] = total_charge;
        }

        // 2. Compute the voltage update for each node.
        let mut delta_vs = vec![0.0; circuit.nodes_count];
        for node in 0..circuit.nodes_count {
            let dv: f64 = roles[node]
                .iter()
                .map(|role| role.delta_v_impact(&charges, damper, node))
                .sum();
            delta_vs[node] = dv;
        }

        // 3. Update the node voltages.
        for node in 0..circuit.nodes_count {
            voltages[node] += delta_vs[node];
        }

        // 4. Compute the maximum absolute voltage change as the error metric.
        let max_delta_v = delta_vs
            .iter()
            .map(|dv| dv.abs())
            .fold(0.0, f64::max);

        // 5. Adaptive damping:
        //    If the error increased compared to the previous iteration,
        //    we consider that the step overshot the correct value.
        //    In that case, we revert the update and reduce the damping factor.
        if max_delta_v > prev_error {
            // Revert to previous voltages.
            voltages = prev_voltages.clone();
            damper *= 0.5;
            // (Optionally, you can log or debug-print the damper adjustment.)
        } else {
            // If the error decreased, store the new voltages and error, and
            // try to gradually increase the damper (up to a maximum of 1.0).
            prev_voltages = voltages.clone();
            prev_error = max_delta_v;
            damper = (damper * 1.1).min(1.0);
        }

        iteration += 1;

        // 6. Check for convergence.
        if max_delta_v < tolerance {
            println!("Converged in {} iterations with final damper {}.", iteration, damper);
            break;
        }

        // If the maximum number of iterations is reached, warn and exit.
        if iteration >= max_iterations {
            println!("Warning: Maximum iterations reached ({} iterations) without convergence.", iteration);
            break;
        }
    }

    // --- Final adjustment ---
    // Normalize voltages relative to the designated ground node.
    let ground_voltage = voltages[circuit.ground_node];
    for v in &mut voltages {
        *v -= ground_voltage;
    }

    voltages
}