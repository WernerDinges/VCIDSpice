use std::f64::consts::E;

/// Enum representing different roles (i.e. component contributions) for a node.
#[derive(Clone, Debug)]
pub enum Role {
    /// Represents a constant current injection into the node.
    ConstantCharge(f64),
    /// Represents a linear (ohmic) contribution from a neighbor node.
    Linear { conductance: f64, neighbor: usize },
    /// Represents a nonlinear exponential contribution (e.g. diode behavior).
    Exponential {
        i_s: f64,
        n: f64,
        neighbor: usize,
        anode: usize,
        cathode: usize,
        flip: f64 // Sign flip to account for diode direction.
    },
}

impl Role {

    /// Computes the contribution of this role to the “virtual charge” at the target node.
    ///
    /// * For resistors, this is simply the voltage difference multiplied by conductance.
    /// * For diodes, an exponential current–voltage characteristic is used.
    /// * Constant charges inject a fixed amount.
    pub fn q_vir_impact(&self, voltages: &[f64], target_node: usize) -> f64 {
        match *self {
            Role::ConstantCharge(current) => current,
            Role::Linear { conductance, neighbor } => {
                // Ohm’s law (with scaling factor t_vir already included in conductance)
                conductance * (voltages[neighbor] - voltages[target_node])
            }
            Role::Exponential { i_s, n, neighbor: _, anode, cathode, flip } => {
                // Compute diode current with voltage clamping to avoid overflow.
                let v_diff = (voltages[anode] - voltages[cathode])
                    .clamp(-5.0, 5.0);
                flip * i_s * (E.powf(v_diff / (n * 0.025852)) - 1.0)
            }
        }
    }

    /// Computes the contribution of this role to the voltage update.
    ///
    /// * For resistor or diode branches, the voltage difference between this node and its neighbor
    /// is used (scaled by the damping factor).
    /// * Constant-charge roles do not contribute to voltage updates.
    pub fn delta_v_impact(&self, charges: &[f64], damper: f64, target_node: usize) -> f64 {
        match *self {
            Role::ConstantCharge(_) => 0.,
            Role::Linear { neighbor, .. } | Role::Exponential { neighbor, .. } => {
                damper * (charges[target_node] - charges[neighbor])
            }
        }
    }

}

// -------------------------------------------------------------------------------------------------

pub fn q_vir_impact(
    voltages: &[f64],
    consts: &ConstantCharges,
    linears: &LinearContributions,
    exps: &ExponentialContributions
) -> Vec<f64> {
    let mut q_vir = vec![0.0; voltages.len()];

    for i in 0..consts.currents.len() {
        let target = consts.target_nodes[i];
        q_vir[target] += consts.currents[i];
    }

    for i in 0..linears.conductances.len() {
        let target = linears.target_nodes[i];
        let neighbor = linears.neighbors[i];
        let g = linears.conductances[i];
        q_vir[target] += g * (voltages[neighbor] - voltages[target]);
    }

    for i in 0..exps.i_s.len() {
        let v_diff = (voltages[exps.anodes[i]] - voltages[exps.cathodes[i]])
            .clamp(-5.0, 5.0);
        let current = exps.flips[i] * exps.i_s[i] * (E.powf(v_diff / (exps.n[i] * 0.025852)) - 1.0);
        let target = exps.target_nodes[i];
        q_vir[target] += current;
    }

    q_vir
}

pub fn delta_v_impact(
    charges: &[f64],
    damper: f64,
    consts: &ConstantCharges,
    linears: &LinearContributions,
    exps: &ExponentialContributions
) -> Vec<f64> {
    let mut delta_v = vec![0.0; consts.currents.len()];

    // Linear
    for i in 0..linears.conductances.len() {
        let target = linears.target_nodes[i];
        let neighbor = linears.neighbors[i];
        delta_v[target] += damper * (charges[target] - charges[neighbor])
    }

    // Exponent
    for i in 0..exps.i_s.len() {
        let target = linears.target_nodes[i];
        let neighbor = linears.neighbors[i];
        delta_v[target] += damper * (charges[target] - charges[neighbor])
    }

    delta_v
}

/// Maintains an association from each node to a list of indices into each container
pub struct NodeRoles {
    pub constant_indices: Vec<usize>,
    pub linear_indices: Vec<usize>,
    pub exponential_indices: Vec<usize>,
}

/// All constant current contributions
pub struct ConstantCharges {
    pub currents: Vec<f64>,
    pub target_nodes: Vec<usize>,
}

/// All linear (resistive) connections
pub struct LinearContributions {
    pub conductances: Vec<f64>,
    pub neighbors: Vec<usize>,
    pub target_nodes: Vec<usize>,
}

/// All exponential (diode-like) connections
pub struct ExponentialContributions {
    pub i_s: Vec<f64>,
    pub n: Vec<f64>,
    pub anodes: Vec<usize>,
    pub cathodes: Vec<usize>,
    pub flips: Vec<f64>,
    pub target_nodes: Vec<usize>,
}