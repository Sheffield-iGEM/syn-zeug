use std::{env, error::Error, fs};
use syn_zeug::seq::{Seq, Kind};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = dbg!(env::args().skip(1).collect());
    let input = fs::read_to_string(&args[0])?;
    let dna = Seq::rna(input.trim())?;
    println!("{}", dna.convert(Kind::Protein)?);
    Ok(())
}
