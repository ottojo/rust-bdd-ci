use std::collections::HashMap;

use boolean_expression::{Expr, BDD};

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

fn main() {
    let allowed = example_expr();
    let items = HashMap::from([
        (RiverCrossing::Fox, false),
        (RiverCrossing::Chicken, true),
        (RiverCrossing::Grain, true),
    ]);
    let mut bdd = BDD::new();
    let allowed_func = bdd.from_expr(&allowed);
    let dot = bdd.to_dot(allowed_func);

    let expr_result = allowed.evaluate(&items);
    let bdd_result = bdd.evaluate(allowed_func, &items);
    assert!(expr_result == bdd_result);

    println!("{}", dot);
    println!("Result: {}", bdd_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_bit_at(input: usize, n: u32) -> bool {
        if n < usize::BITS {
            input & (1 << n) != 0
        } else {
            false
        }
    }

    #[test]
    fn test_expr() {
        let allowed = example_expr();
        // Truth table for inputs Grain, Chicken, Fox:
        //                                       000    001   010   011    100   101   110    111
        let results = vec![false, false, true, true, false, true, true, false];

        for i in 0usize..8usize {
            let items = HashMap::from([
                (RiverCrossing::Fox, get_bit_at(i, 0)),
                (RiverCrossing::Chicken, get_bit_at(i, 1)),
                (RiverCrossing::Grain, get_bit_at(i, 2)),
            ]);
            let expr_result = allowed.evaluate(&items);
            assert!(expr_result == results[i])
        }
    }

    #[test]
    fn test_bdd() {
        let allowed = example_expr();
        // Truth table for inputs Grain, Chicken, Fox:
        //                                       000    001   010   011    100   101   110    111
        let results = vec![false, false, true, true, false, true, true, false];
        let mut bdd = BDD::new();
        let allowed_func = bdd.from_expr(&allowed);

        for i in 0usize..8usize {
            let items = HashMap::from([
                (RiverCrossing::Fox, get_bit_at(i, 0)),
                (RiverCrossing::Chicken, get_bit_at(i, 1)),
                (RiverCrossing::Grain, get_bit_at(i, 2)),
            ]);

            let bdd_result = bdd.evaluate(allowed_func, &items);
            assert!(bdd_result == results[i])
        }
    }
}
