
use brainfrsck::prelude::*;

fn main() {
    let sum: &'static str = ",>,<[[[->+<],],]";

    eprintln!("{}", eval_string(sum, Some((1..6).collect())).unwrap().to_vec()[0]);
}