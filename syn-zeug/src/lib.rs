use bio::alphabets::{dna, protein, rna, Alphabet};
use std::{
    collections::HashMap,
    fmt,
    ops::{Index, IndexMut},
    str::from_utf8,
};
// Keep an eye on this: https://github.com/rust-lang/rust/issues/74465
use once_cell::sync::Lazy;

static ALPHABETS: Lazy<HashMap<SeqKind, Alphabet>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(SeqKind::Dna, dna::iupac_alphabet());
    m.insert(SeqKind::Rna, rna::iupac_alphabet());
    m.insert(SeqKind::Protein, protein::iupac_alphabet());
    m
});

// TODO: Create a set of Enums for describing nucleotides like:
// #[repr(u8)]
// enum Dna {
//      A = ...,
//      C,
//      G,
//      T,
// }
// Then impl From<Nucleotide> for u8 and allow users to specify a nucleotide
// enum that's automatically converted into a byte!

// TODO: Would references / slices be better here?
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Seq {
    bytes: Vec<u8>,
    kind: SeqKind,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SeqKind {
    Dna,
    Rna,
    Protein,
}

impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Would it be faster to use `from_utf8_unchecked()`? Do I care?
        f.write_str(from_utf8(&self.bytes).expect("Seq did not contain valid UTF-8"))
    }
}

// TODO: Add an `rna()` constructor + tests + conversion / complementing code
impl Seq {
    // TODO: I should probably create a custom error type instead us using a string!
    pub fn dna(seq: impl AsRef<[u8]>) -> Result<Self, String> {
        let seq = seq.as_ref();
        // TODO: `.is_word()` could be improved to return the first non-word byte
        if ALPHABETS[&SeqKind::Dna].is_word(seq) {
            Ok(Self {
                bytes: seq.to_vec(),
                kind: SeqKind::Dna,
            })
        } else {
            Err(String::from("The provided sequence was not valid DNA"))
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn count_elements(&self) -> ByteMap<usize> {
        let mut counts = ByteMap::default();
        for &b in &self.bytes {
            counts[b] += 1;
        }
        counts
    }

    pub fn reverse_complement(&self) -> Result<Self, String> {
        match self.kind {
            SeqKind::Dna => Ok(Self {
                bytes: dna::revcomp(&self.bytes),
                ..*self
            }),
            _ => todo!(),
        }
    }

    pub fn convert(&self, kind: SeqKind) -> Result<Self, String> {
        match (self.kind, kind) {
            (SeqKind::Dna, SeqKind::Rna) => Ok(Self {
                bytes: self
                    .bytes
                    .iter()
                    .map(|&b| if b == b'T' || b == b't' { b + 1 } else { b })
                    .collect(),
                kind: SeqKind::Rna,
            }),
            _ => todo!(),
        }
    }
}

// TODO: Split things into a couple of modules
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ByteMap<T>([T; 128]);

impl<T: Copy> ByteMap<T> {
    pub fn new(default: T) -> Self {
        Self([default; 128])
    }
}

impl<T: Copy + Default> Default for ByteMap<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Index<u8> for ByteMap<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<u8> for ByteMap<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_valid_dna_sequence() {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA");
        assert!(dna.is_ok());
        assert_eq!(dna.unwrap().kind, SeqKind::Dna);
    }

    #[test]
    fn read_invalid_dna_sequence() {
        let dna = Seq::dna("AGCTTTXCATTCTGACNGCA");
        assert_eq!(
            dna,
            Err(String::from("The provided sequence was not valid DNA"))
        );
    }

    #[test]
    fn dna_to_string() -> Result<(), String> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.to_string(), String::from("AGCTTTTCATTCTGACTGCA"));
        Ok(())
    }

    #[test]
    fn get_sequence_length() -> Result<(), String> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.len(), 20);
        Ok(())
    }

    #[test]
    fn is_sequence_empty() -> Result<(), String> {
        let dna = Seq::dna("")?;
        assert!(dna.is_empty());
        let dna = Seq::dna("ACGT")?;
        assert!(!dna.is_empty());
        Ok(())
    }

    #[test]
    fn count_nucleotides() -> Result<(), String> {
        let dna =
            Seq::dna("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC")?;
        let counts = dna.count_elements();
        assert_eq!(counts[b'A'], 20);
        assert_eq!(counts[b'C'], 12);
        assert_eq!(counts[b'G'], 17);
        assert_eq!(counts[b'T'], 21);
        Ok(())
    }

    #[test]
    fn dna_to_rna() -> Result<(), String> {
        let dna = Seq::dna("GATGGAACTTGACTACGTAAATT")?;
        let rna = dna.convert(SeqKind::Rna)?;
        assert_eq!(rna.bytes, b"GAUGGAACUUGACUACGUAAAUU");
        assert_eq!(rna.kind, SeqKind::Rna);
        Ok(())
    }

    #[test]
    fn dna_to_rna_keep_case() -> Result<(), String> {
        let dna = Seq::dna("GaTgGaAcTtGaCtAcGtAaAtT")?;
        let rna = dna.convert(SeqKind::Rna)?;
        // TODO: Write Seq::rna() and make this one assert that looks like:
        // assert_eq!(rna, Seq::rna("..."));
        assert_eq!(rna.bytes, b"GaUgGaAcUuGaCuAcGuAaAuU");
        assert_eq!(rna.kind, SeqKind::Rna);
        Ok(())
    }

    #[test]
    fn reverse_complement_dna() -> Result<(), String> {
        let dna = Seq::dna("AAAACCCGGT")?;
        assert_eq!(dna.reverse_complement()?.bytes, b"ACCGGGTTTT");
        Ok(())
    }

    #[test]
    fn reverse_complement_dna_keep_case() -> Result<(), String> {
        let dna = Seq::dna("aaaacCCGGT")?;
        assert_eq!(dna.reverse_complement()?.bytes, b"ACCGGgtttt");
        Ok(())
    }
}
