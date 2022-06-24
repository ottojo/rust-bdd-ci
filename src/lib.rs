use boolean_expression::{Expr, BDD};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum RiverCrossing {
    Chicken,
    Fox,
    Grain,
}

pub fn example_expr() -> Expr<RiverCrossing> {
    let chicken = Expr::Terminal(RiverCrossing::Chicken);
    let fox_and_grain = Expr::Terminal(RiverCrossing::Fox) & Expr::Terminal(RiverCrossing::Grain);

    (!chicken.clone() & fox_and_grain.clone()) | (chicken & !fox_and_grain)
}

pub fn xor_direct(n: u64) -> BDD<u64> {
    let mut b = BDD::new();
    let expr = Expr::xor_direct((1..n).collect());
    b.from_expr(&expr);
    b
}

pub fn xor_ite(n: u64) -> BDD<u64> {
    let mut b = BDD::new();
    let expr = Expr::xor_ite((1..n).collect());
    b.from_expr(&expr);
    b
}
