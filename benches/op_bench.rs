use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vcid_spice::general::circuit::Circuit;
use vcid_spice::general::circuit::Component::{CurrentDc, Diode, Resistor};
use vcid_spice::simulation::op::simulate_op;

fn op_simulation_benchmark(c: &mut Criterion) {
    c.bench_function("op_simulation_small", |b| {
        // Create a small test circuit
        let n = 3;
        let mut circuit = Circuit::new(n, 0);
        circuit.add_component(CurrentDc { anode: 2, cathode: 0, current: 1.0 });
        circuit.add_component(Resistor { pin1: 0, pin2: 1, r: 5.0 });
        circuit.add_component(Diode { anode: 2, cathode: 1, i_s: 170e-9, n: 2.0 });
        circuit.add_component(Resistor { pin1: 0, pin2: 2, r: 5.0 });

        b.iter(|| {
            simulate_op(black_box(&circuit), black_box(0.05), black_box(1e-3), None)
        });
    });
}

criterion_group!(benches, op_simulation_benchmark);
criterion_main!(benches); 