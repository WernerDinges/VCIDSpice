# VCID Spice (Virtual Charge Iterative Diffusion)

A circuit simulation library written in Rust.

## Features

- Operating point (DC) circuit analysis
- Support for non-linear components (diodes)
- Easy-to-use API for circuit definition

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vcid_spice = "0.1.0"
```

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

## License

Licensed under [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions. 