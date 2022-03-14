use syn_zeug::*;
use std::{fs, env};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let input = fs::read_to_string(&args[0]).unwrap();
    let dna: Vec<Dna> = read_sequence(input.trim()).unwrap();
    dbg!(count_components(&dna));
}
