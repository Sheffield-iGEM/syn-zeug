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

pub const CODON_TABLE: Map<&[u8], u8> = phf_map! {
    b"UUU" => b'F',
    b"UUC" => b'F',
    b"UUA" => b'L',
    b"UUG" => b'L',

    b"UCU" => b'S',
    b"UCC" => b'S',
    b"UCA" => b'S',
    b"UCG" => b'S',

    b"UAU" => b'Y',
    b"UAC" => b'Y',
    b"UAA" => b'*',
    b"UAG" => b'*',

    b"UGU" => b'C',
    b"UGC" => b'C',
    b"UGA" => b'*',
    b"UGG" => b'W',

    b"CUU" => b'L',
    b"CUC" => b'L',
    b"CUA" => b'L',
    b"CUG" => b'L',

    b"CCU" => b'P',
    b"CCC" => b'P',
    b"CCA" => b'P',
    b"CCG" => b'P',

    b"CAU" => b'H',
    b"CAC" => b'H',
    b"CAA" => b'Q',
    b"CAG" => b'Q',

    b"CGU" => b'R',
    b"CGC" => b'R',
    b"CGA" => b'R',
    b"CGG" => b'R',

    b"AUU" => b'I',
    b"AUC" => b'I',
    b"AUA" => b'I',
    b"AUG" => b'M',

    b"ACU" => b'T',
    b"ACC" => b'T',
    b"ACA" => b'T',
    b"ACG" => b'T',

    b"AAU" => b'N',
    b"AAC" => b'N',
    b"AAA" => b'K',
    b"AAG" => b'K',

    b"AGU" => b'S',
    b"AGC" => b'S',
    b"AGA" => b'R',
    b"AGG" => b'R',

    b"GUU" => b'V',
    b"GUC" => b'V',
    b"GUA" => b'V',
    b"GUG" => b'V',

    b"GCU" => b'A',
    b"GCC" => b'A',
    b"GCA" => b'A',
    b"GCG" => b'A',

    b"GAU" => b'D',
    b"GAC" => b'D',
    b"GAA" => b'E',
    b"GAG" => b'E',

    b"GGU" => b'G',
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
