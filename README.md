# VCIDSpice

VCIDSpice (short for Virtual Charge Iterative Distribution) is my small attempt at simulating
electrical circuits without solving matrices.
Here is only a brief exposition of how it works:

The developed method is based on the law of conservation of electric charge.
For each node, an initial assumption is made about its voltage
(default 0 for all nodes except voltage sources; can be set manually for faster convergence).
Then iterations are performed, consisting of the following steps:
- For each node, the instantaneous currents coming from neighboring nodes are calculated
  on the basis of the current iteration voltages and considered as electric charges “accumulated” in the node.
  Let us call such charges virtual charges;
- Based on the virtual charges, the voltage value at each node is updated for each node;
- The updated voltages are used as input data for the next iteration.

Circuit components are defined by their “roles” - i.e., how they affect virtual charge diffusion
and voltage conversion. This makes it convenient to integrate black box models.

Also, the inherently iterative nature of the method relieves the need to linearize the system of nonlinear equations.

## Available features

- Constant current sources, resistors, diodes
- Operating point (DC) circuit analysis
- Support for non-linear components (diodes)
- API for circuit definition

## In the near future

- Constant voltage sources
- Transient analysis
- Capacities
- Inductances

## Example Usage

```rust
use vcid_spice::general::circuit::Circuit;
use vcid_spice::general::circuit::Component::{CurrentDc, Diode, Resistor};
use vcid_spice::simulation::op::simulate_op;

fn main() {
    // Create a circuit with 3 nodes and ground reference
    let mut circuit = Circuit::new(3, 0);

    // Add components
    circuit.add_component(CurrentDc { anode: 2, cathode: 0, current: 1.0 });
    circuit.add_component(Resistor { pin1: 0, pin2: 1, r: 5.0 });
    circuit.add_component(Diode { anode: 2, cathode: 1, i_s: 170e-9, n: 2.0 });
    circuit.add_component(Resistor { pin1: 0, pin2: 2, r: 5.0 });

    // Simulate operating point
    let voltages = simulate_op(&circuit, 0.05, 1e-3, None);
    println!("Node voltages: {:?}", voltages);
}
```

## Theory

First, let's take a closer look at the solution mechanism.
Based on the assumptions about voltages for each node, its virtual charge for the current iteration is calculated:

$$Q_{vir_i}^{(k+1)} = \sum_j G_{ij} \left( V_j^{(k)} - V_i^{(k)} \right) \tau_{vir}$$

where $\tau_{vir}$ is the virtual time step, a simple coefficient set for the whole simulation.
It is responsible for the overall “softness” of the voltage values update.

Then, based on the calculated virtual charges, the voltages of each node
(except for fixed potentials, e.g., ground) are updated:

$$V_i^{(k+1)} = V_i^{(k)} + \alpha^{(k)} \sum_j S_{vir_{ij}} \left( Q_{vir_i}^{(k+1)} - Q_{vir_j}^{(k+1)} \right)$$

The voltages of $V_i^{(k)}$ nodes are updated at each iteration using virtual charges
and a virtual elastance $S_{vir_{ij}}$ between two nodes, which controls how sensitive
the changes in potential are to the charge difference between the nodes
(the virtual elastance is not related to the real capacitances in the circuit;
it is only related to the virtual charges). The damping factor $\alpha^{(k)}$ is determined dynamically
for all nodes to control the stiffness of the voltage updates at different stages of the simulation.

## License

Licensed under [MIT license](http://opensource.org/licenses/MIT)