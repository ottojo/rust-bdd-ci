use boolean_expression::{Expr, BDD};
use criterion::{criterion_group, criterion_main, Criterion};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
enum RiverCrossing {
    Chicken,
    Fox,
    Grain,
}

fn example_expr() -> Expr<RiverCrossing> {
    let chicken = Expr::Terminal(RiverCrossing::Chicken);
    let fox_and_grain = Expr::Terminal(RiverCrossing::Fox) & Expr::Terminal(RiverCrossing::Grain);

    let allowed = (!chicken.clone() & fox_and_grain.clone()) | (chicken & !fox_and_grain);
    allowed
}

fn bench_build_bdd(c: &mut Criterion) {
    let allowed = example_expr();

    c.bench_function("bdd from expr", |b| {
        b.iter(|| {
            let mut bdd = BDD::new();
            let _allowed_func = bdd.from_expr(&allowed);
        })
    });
}

criterion_group!(benches, bench_build_bdd);
criterion_main!(benches);
