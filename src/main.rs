mod naive_solution;
mod optimized;
mod common;

use naive_solution::*;
use common::*;

fn main() {
    let str = "abcdefgijklmnopqrstuvwxyz";
    let res = NaiveSolution::solve(str);

    println!("{}", res);
}
