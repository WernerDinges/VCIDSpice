# VCID Spice Usage Guide

This document provides a comprehensive guide to using the VCID Spice circuit simulation library.

## Table of Contents

1. [Installation](#installation)
2. [Creating a Circuit](#creating-a-circuit)
3. [Adding Components](#adding-components)
4. [Running Simulations](#running-simulations)
5. [Working with Results](#working-with-results)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vcid_spice = "0.1.0"
```

Then import the required modules in your Rust code:

```rust
use vcid_spice::general::circuit::Circuit;
use vcid_spice::general::circuit::Component;
use vcid_spice::simulation::op::simulate_op;
```

## Creating a Circuit

To create a new circuit, specify the number of nodes and the ground reference node:

```rust
// Create a circuit with 5 nodes (0-4), with node 0 as ground
let mut circuit = Circuit::new(5, 0);
```

## Adding Components

VCID Spice supports various circuit components:

### Resistors

```rust
use vcid_spice::general::circuit::Component::Resistor;

// Add a 1k resistor between nodes 1 and 2
circuit.add_component(Resistor {
    pin1: 1,
    pin2: 2,
    r: 1000.0 // resistance in ohms
});
```

### Diodes

```rust
use vcid_spice::general::circuit::Component::Diode;

// Add a diode from node 2 to node 3
circuit.add_component(Diode {
    anode: 2,
    cathode: 3,
    i_s: 1e-12, // saturation current in amperes
    n: 1.0      // ideality factor
});
```

### DC Current Sources

```rust
use vcid_spice::general::circuit::Component::CurrentDc;

// Add a 1mA current source from node 4 to ground
circuit.add_component(CurrentDc {
    anode: 4,
    cathode: 0,
    current: 1e-3 // current in amperes
});
```

## Running Simulations

### Operating Point (DC) Analysis

```rust
let step_size = 0.01;       // Newton-Raphson step size
let error_tolerance = 1e-6;  // Convergence tolerance
let max_iterations = Some(100); // Maximum iterations

// Run the simulation
let node_voltages = simulate_op(&circuit, step_size, error_tolerance, max_iterations);

// Print results
for (i, voltage) in node_voltages.iter().enumerate() {
    println!("Node {}: {} V", i, voltage);
}
```

## Working with Results

The simulation results are returned as a vector of node voltages. The index corresponds to the node number.

```rust
// Get the voltage at node 3
let node3_voltage = node_voltages[3];

// Calculate voltage difference between nodes
let voltage_diff = node_voltages[2] - node_voltages[1];
```

For more examples, check the `examples` directory in the repository. 