use criterion::{black_box, criterion_group, criterion_main, Criterion};
use edvac::{inst, Edvac};

// The target here is for the average addition time to be under 864 μs (which is
// the average addition speed of the original machine). Needless to say, we blow
// this out of the water.
fn benchmark(c: &mut Criterion) {
    c.bench_function("addition", |b| {
        let mut computer = Edvac::default();
        // one of the sources (I unfortunately forget which- if you know, please
        // file an issue) that I used described *exactly* how the addition times
        // were measured- for us this doesn't really matter for us, because time
        // taken for each addition is (more or less) independent of the location
        // in memory which was not the case for the original machine.
        computer.high_speed_memory.load(vec![
            (0o0000, inst!(A 0o0001 0o1000 0o0001 0o0000)),
            (0o1000, 1.try_into().unwrap()),
        ]);

        b.iter(|| computer.step_once());
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
