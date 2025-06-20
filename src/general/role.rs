use std::f64::consts::E;

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