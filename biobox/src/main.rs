use std::{env, error::Error, fs};
use syn_zeug::seq::{Kind, Seq};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = dbg!(env::args().skip(1).collect());
    let input = fs::read_to_string(&args[0])?;
    let dna = Seq::dna(input.trim())?;
    for (_, seq) in dna.find_orfs(1) {
        println!("{}", seq.convert(Kind::Protein)?);
    }
    Ok(())
}
