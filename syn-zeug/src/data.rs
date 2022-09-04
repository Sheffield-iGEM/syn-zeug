use bio::alphabets::{dna, protein, rna};
use phf::{phf_map, Map};
use std::{collections::HashMap, iter};
// TODO: Keep an eye on this: https://github.com/rust-lang/rust/issues/74465
use once_cell::sync::Lazy;

use crate::{
    seq::{Alphabet, Kind},
    utils::expand_iupac,
};

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

// TODO: I might need to add a gap character (`.` or `-`) to these alphabets someday
pub static ALPHABET_MAP: Lazy<HashMap<(Kind, Alphabet), bio::alphabets::Alphabet>> =
    Lazy::new(|| {
        let stop_codon = bio::alphabets::Alphabet::new(b"*");
        HashMap::from([
            ((Kind::Dna, Alphabet::Base), dna::alphabet()),
            ((Kind::Dna, Alphabet::N), dna::n_alphabet()),
            ((Kind::Dna, Alphabet::Iupac), dna::iupac_alphabet()),
            ((Kind::Rna, Alphabet::Base), rna::alphabet()),
            ((Kind::Rna, Alphabet::N), rna::n_alphabet()),
            ((Kind::Rna, Alphabet::Iupac), rna::iupac_alphabet()),
            (
                (Kind::Protein, Alphabet::Base),
                protein::alphabet().union(&stop_codon),
            ),
            (
                (Kind::Protein, Alphabet::Iupac),
                protein::iupac_alphabet().union(&stop_codon),
            ),
        ])
    });

// TODO: Write a test for the sub/superset properties here!
pub const IUPAC_DNA: Map<u8, &[u8]> = phf_map! {
    b'R' => b"AG",
    b'Y' => b"CT",
    b'S' => b"GC",
    b'W' => b"AT",
    b'K' => b"GT",
    b'M' => b"AC",

    b'B' => b"CGTYSK",
    b'D' => b"AGTRWK",
    b'H' => b"ACTYWM",
    b'V' => b"ACGRSM",

    b'N' => b"ACGTRYSWKMBDHV",
};

pub const IUPAC_RNA: Map<u8, &[u8]> = phf_map! {
    b'R' => b"AG",
    b'Y' => b"CU",
    b'S' => b"GC",
    b'W' => b"AU",
    b'K' => b"GU",
    b'M' => b"AC",

    b'B' => b"CGUYSK",
    b'D' => b"AGURWK",
    b'H' => b"ACUYWM",
    b'V' => b"ACGRSM",

    b'N' => b"ACGURYSWKMBDHV",
};

// TODO: Don't know if I should use Biopython's definition or the smaller rust-bio version with
// just B, Z, and X added
pub const IUPAC_PROTEIN: Map<u8, &[u8]> = phf_map! {
    b'B' => b"DN",
    b'Z' => b"EQ",

    b'X' => b"ABCDEFGHIKLMNPQRSTVWYZ",
};

// TODO: Rename this to: IUPAC_CODON_TABLE and only use when needed!
pub static CODON_TABLE: Lazy<HashMap<Vec<u8>, u8>> = Lazy::new(|| {
    let residue_codons = |(codon, residue)| -> Vec<_> {
        expand_iupac(codon, &IUPAC_RNA)
            .into_iter()
            .zip(iter::repeat(residue))
            .collect()
    };

    let mut table = HashMap::with_capacity(3375);

    let codons = [
        (b"NNN", b'X'),
        (b"RAY", b'B'),
        (b"SAR", b'Z'),
        (b"GCN", b'A'),
        (b"UGY", b'C'),
        (b"GAY", b'D'),
        (b"GAR", b'E'),
        (b"UUY", b'F'),
        (b"GGN", b'G'),
        (b"CAY", b'H'),
        (b"AUH", b'I'),
        (b"AAR", b'K'),
        (b"CUN", b'L'),
        (b"YUR", b'L'),
        (b"AUG", b'M'),
        (b"AAY", b'N'),
        (b"CCN", b'P'),
        (b"CAR", b'Q'),
        (b"CGN", b'R'),
        (b"MGR", b'R'),
        (b"UCN", b'S'),
        (b"AGY", b'S'),
        (b"ACN", b'T'),
        (b"GUN", b'V'),
        (b"UGG", b'W'),
        (b"UAY", b'Y'),
        (b"UAR", b'*'),
        (b"URA", b'*'),
    ];

    table.extend(codons.into_iter().flat_map(residue_codons));
    table
});

pub const OLD_CODON_TABLE: Map<&[u8], u8> = phf_map! {
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
        assert_eq!(CODON_TABLE.len(), 3375);
        let mut values: Vec<_> = CODON_TABLE.values().collect();
        values.sort_unstable();
        values.dedup();
        assert_eq!(values.len(), 24);
    }
}
