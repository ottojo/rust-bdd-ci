use std::time::Duration;

use boolean_expression::BDD;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_bdd_ci::{example_expr, xor_direct, xor_ite};

#[allow(dead_code)]
fn bench_xor(c: &mut Criterion) {
    let mut group = c.benchmark_group("xor");
    group.sample_size(10);
    group.warm_up_time(Duration::new(1, 0));

    for i in [3, 5, 10, 50, 75, 100, 125, 150].iter() {
        group.bench_with_input(BenchmarkId::new("Direct", i), i, |b, i| {
            b.iter(|| xor_direct(*i))
        });
        group.bench_with_input(BenchmarkId::new("ITE", i), i, |b, i| b.iter(|| xor_ite(*i)));
    }
    group.finish();
}

fn bench_build_bdd(c: &mut Criterion) {
    let allowed = example_expr();

    c.bench_function("bdd from expr", |b| {
        b.iter(|| {
            let mut bdd = BDD::new();
            bdd.from_expr(&allowed)
        })
    });
}

criterion_group!(benches, bench_build_bdd /*bench_xor*/,);
criterion_main!(benches);
