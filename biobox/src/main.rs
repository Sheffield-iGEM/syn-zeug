use std::{env, fs};
use syn_zeug::*;

fn main() {
    let args: Vec<_> = dbg!(env::args().skip(1).collect());
    let input = fs::read_to_string(&args[0]).unwrap();
    let dna = Seq::dna(input.trim()).unwrap();
    dbg!(dna.count_elements());
}
