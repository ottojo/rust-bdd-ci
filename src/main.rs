use std::env;

use rust_bdd_ci::{xor_direct, xor_ite};

fn main() {
    let n = env::args()
        .nth(1)
        .expect("Please specify XOR size")
        .parse()
        .unwrap();

    if n != 0 {
        let ite_bdd = xor_ite(n);
        let direct_bdd = xor_direct(n);
        println!("{} {}", ite_bdd.nodes(), direct_bdd.nodes());
        return;
    }

    println!("# Nr. Options, Nodes using ITE, Nodes using XOR-Ladder");
    for i in 3..150 {
        let ite_bdd = xor_ite(i);
        let direct_bdd = xor_direct(i);
        println!("{} {} {}", i, ite_bdd.nodes(), direct_bdd.nodes());
    }
}
