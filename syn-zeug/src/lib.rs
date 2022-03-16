use bio::alphabets::{self, Alphabet};
use std::collections::HashMap;
// Keep an eye on this: https://github.com/rust-lang/rust/issues/74465
use once_cell::sync::Lazy;

static ALPHABETS: Lazy<HashMap<SeqKind, Alphabet>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(SeqKind::Dna, alphabets::dna::iupac_alphabet());
    m.insert(SeqKind::Rna, alphabets::rna::iupac_alphabet());
    m.insert(SeqKind::Protein, alphabets::protein::iupac_alphabet());
    m
});

// FIXME: Would references / slices be better here?
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

impl Seq {
    // FIXME: I should probably create a custom error type instead us using a string!
    pub fn dna(seq: impl AsRef<[u8]>) -> Result<Self, String> {
        let seq = seq.as_ref();
        // FIXME: `.is_word()` could be improved to return the first non-word byte
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

    pub fn count_elements(&self) -> [usize; 128] {
        let mut counts = [0; 128];
        for &b in &self.bytes {
            counts[b as usize] += 1;
        }
        counts
    }

    pub fn count_bases(&self) -> [usize; 4] {
        let (mut a, mut c, mut g, mut t) = (0, 0, 0, 0);
        for &base in &self.bytes {
            if base == b'A' {
                a += 1;
            } else if base == b'C' {
                c += 1;
            } else if base == b'G' {
                g += 1;
            } else if base == b'T' {
                t += 1;
            }
        }
        [a, c, g, t]
    }

    pub fn convert(&self, kind: SeqKind) -> Self {
        match (self.kind, kind) {
            (SeqKind::Dna, SeqKind::Rna) => Self {
                bytes: self
                    .bytes
                    .iter()
                    .map(|&b| if b == b'T' || b == b't' { b + 1 } else { b })
                    .collect(),
                kind: SeqKind::Rna,
            },
            _ => todo!(),
        }
    }
}

pub struct SparseArray<T, const N: usize> {
    inner: [T; N],
    offset: usize,
}

impl<T: Copy, const N: usize> SparseArray<T, N> {
    // FIXME: Silly. This needs to eventually use Default
    pub fn new(value: T, offset: usize) -> Self {
        Self {
            inner: [value; N],
            offset,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_valid_dna_sequence() {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA");
        assert!(dna.is_ok());
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
        assert_eq!(counts[b'A' as usize], 20);
        assert_eq!(counts[b'C' as usize], 12);
        assert_eq!(counts[b'G' as usize], 17);
        assert_eq!(counts[b'T' as usize], 21);
        Ok(())
    }

    #[test]
    fn dna_to_rna() -> Result<(), String> {
        let dna = Seq::dna("GATGGAACTTGACTACGTAAATT")?;
        assert_eq!(dna.convert(SeqKind::Rna).bytes, b"GAUGGAACUUGACUACGUAAAUU");
        Ok(())
    }

    #[test]
    fn dna_to_rna_keep_case() -> Result<(), String> {
        let dna = Seq::dna("GaTgGaAcTtGaCtAcGtAaAtT")?;
        assert_eq!(dna.convert(SeqKind::Rna).bytes, b"GaUgGaAcUuGaCuAcGuAaAuU");
        Ok(())
    }
}
