use std::collections::HashMap;

use boolean_expression::BDD;
use rust_bdd_ci::{example_expr, RiverCrossing};

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
