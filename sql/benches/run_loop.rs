use criterion::{criterion_group, criterion_main, Criterion};

fn bench_run_loop(c: &mut Criterion) {
    c.bench_function("bench_run_loop", |b| {
        b.iter(|| {
            let mut a = 0;
            std::hint::black_box(for i in 1..=1000 {
                a = a + i;
            });
        });
    });
}

criterion_group!(benches, bench_run_loop,);
criterion_main!(benches);
