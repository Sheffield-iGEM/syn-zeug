use bio::alphabets::{dna, protein, rna};
use phf::{phf_map, Map};
use std::collections::HashMap;
// TODO: Keep an eye on this: https://github.com/rust-lang/rust/issues/74465
use once_cell::sync::Lazy;

use crate::seq::{Alphabet, Kind};

pub const ALPHABETS: [(Kind, Alphabet); 8] = [
    (Kind::Dna, Alphabet::Base),
    (Kind::Rna, Alphabet::Base),
    (Kind::Dna, Alphabet::N),
    (Kind::Rna, Alphabet::N),
    (Kind::Protein, Alphabet::Base),
    (Kind::Dna, Alphabet::Iupac),
    (Kind::Rna, Alphabet::Iupac),
    (Kind::Protein, Alphabet::Iupac),
];

pub static ALPHABET_MAP: Lazy<HashMap<(Kind, Alphabet), bio::alphabets::Alphabet>> =
    Lazy::new(|| {
        HashMap::from([
            ((Kind::Dna, Alphabet::Base), dna::alphabet()),
            ((Kind::Dna, Alphabet::N), dna::n_alphabet()),
            ((Kind::Dna, Alphabet::Iupac), dna::iupac_alphabet()),
            ((Kind::Rna, Alphabet::Base), rna::alphabet()),
            ((Kind::Rna, Alphabet::N), rna::n_alphabet()),
            ((Kind::Rna, Alphabet::Iupac), rna::iupac_alphabet()),
            ((Kind::Protein, Alphabet::Base), protein::alphabet()),
            ((Kind::Protein, Alphabet::Iupac), protein::iupac_alphabet()),
        ])
    });

pub const CODON_TABLE: Map<&[u8; 3], u8> = phf_map! {
    b"TTT" => b'F',
    b"TTC" => b'F',
    b"TTA" => b'L',
    b"TTG" => b'L',

    b"TCT" => b'S',
    b"TCC" => b'S',
    b"TCA" => b'S',
    b"TCG" => b'S',

    b"TAT" => b'Y',
    b"TAC" => b'Y',
    b"TAA" => b'*',
    b"TAG" => b'*',

    b"TGT" => b'C',
    b"TGC" => b'C',
    b"TGA" => b'*',
    b"TGG" => b'W',

    b"CTT" => b'L',
    b"CTC" => b'L',
    b"CTA" => b'L',
    b"CTG" => b'L',

    b"CCT" => b'P',
    b"CCC" => b'P',
    b"CCA" => b'P',
    b"CCG" => b'P',

    b"CAT" => b'H',
    b"CAC" => b'H',
    b"CAA" => b'Q',
    b"CAG" => b'Q',

    b"CGT" => b'R',
    b"CGC" => b'R',
    b"CGA" => b'R',
    b"CGG" => b'R',

    b"ATT" => b'I',
    b"ATC" => b'I',
    b"ATA" => b'I',
    b"ATG" => b'M',

    b"ACT" => b'T',
    b"ACC" => b'T',
    b"ACA" => b'T',
    b"ACG" => b'T',

    b"AAT" => b'N',
    b"AAC" => b'N',
    b"AAA" => b'K',
    b"AAG" => b'K',

    b"AGT" => b'S',
    b"AGC" => b'S',
    b"AGA" => b'R',
    b"AGG" => b'R',

    b"GTT" => b'V',
    b"GTC" => b'V',
    b"GTA" => b'V',
    b"GTG" => b'V',

    b"GCT" => b'A',
    b"GCC" => b'A',
    b"GCA" => b'A',
    b"GCG" => b'A',

    b"GAT" => b'D',
    b"GAC" => b'D',
    b"GAA" => b'E',
    b"GAG" => b'E',

    b"GGT" => b'G',
    b"GGC" => b'G',
    b"GGA" => b'G',
    b"GGG" => b'G',
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codon_table_right_size() {
        assert_eq!(CODON_TABLE.len(), 64);
        let mut values: Vec<_> = CODON_TABLE.values().collect();
        values.sort_unstable();
        values.dedup();
        assert_eq!(values.len(), 21);
    }
}
