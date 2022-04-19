use boolean_expression::Expr;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum RiverCrossing {
    Chicken,
    Fox,
    Grain,
}

pub fn example_expr() -> Expr<RiverCrossing> {
    let chicken = Expr::Terminal(RiverCrossing::Chicken);
    let fox_and_grain = Expr::Terminal(RiverCrossing::Fox) & Expr::Terminal(RiverCrossing::Grain);

    let allowed = (!chicken.clone() & fox_and_grain.clone()) | (chicken & !fox_and_grain);
    allowed
}
