use std::f64::consts::E;

/// Enum representing different roles (i.e. component contributions) for a node.
#[derive(Clone, Debug)]
pub(crate) enum Role {
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
    pub(crate) fn q_vir_impact(&self, voltages: &[f64], target_node: usize) -> f64 {
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
    pub(crate) fn delta_v_impact(&self, charges: &[f64], damper: f64, target_node: usize) -> f64 {
        match *self {
            Role::ConstantCharge(_) => 0.,
            Role::Linear { neighbor, .. } | Role::Exponential { neighbor, .. } => {
                damper * (charges[target_node] - charges[neighbor])
            }
        }
    }

}