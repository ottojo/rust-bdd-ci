use boolean_expression::BDD;
use criterion::{criterion_group, criterion_main, Criterion};
use rust_bdd_ci::example_expr;

fn bench_build_bdd(c: &mut Criterion) {
    let allowed = example_expr();

    c.bench_function("bdd from expr", |b| {
        b.iter(|| {
            let mut bdd = BDD::new();
            bdd.from_expr(&allowed)
        })
    });
}

criterion_group!(benches, bench_build_bdd);
criterion_main!(benches);
