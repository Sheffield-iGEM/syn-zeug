use bio::alphabets::{dna, rna};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, str::from_utf8};

use crate::data::{ByteMap, ALPHABETS};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum SeqError {
    InvalidConversion(SeqKind, SeqKind),
    InvalidKind(SeqKind),
    RevComp(SeqKind),
    Invalid,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum SeqKind {
    Dna,
    Rna,
    Protein,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct Seq {
    bytes: Vec<u8>,
    kind: SeqKind,
}

impl Seq {
    fn new_with_kind(seq: impl AsRef<[u8]>, kind: SeqKind) -> Result<Self, SeqError> {
        let seq = seq.as_ref();
        if ALPHABETS[&kind].is_word(seq) {
            Ok(Self {
                bytes: seq.to_vec(),
                kind,
            })
        } else {
            Err(SeqError::InvalidKind(kind))
        }
    }

    pub fn new(seq: impl AsRef<[u8]>) -> Result<Self, SeqError> {
        Self::dna(&seq)
            .or_else(|_| Self::rna(&seq))
            .or_else(|_| Self::protein(&seq))
            .map_err(|_| SeqError::Invalid)
    }

    pub fn dna(seq: impl AsRef<[u8]>) -> Result<Self, SeqError> {
        Self::new_with_kind(seq, SeqKind::Dna)
    }

    pub fn rna(seq: impl AsRef<[u8]>) -> Result<Self, SeqError> {
        Self::new_with_kind(seq, SeqKind::Rna)
    }

    pub fn protein(seq: impl AsRef<[u8]>) -> Result<Self, SeqError> {
        Self::new_with_kind(seq, SeqKind::Protein)
    }

    pub fn kind(&self) -> SeqKind {
        self.kind
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

    pub fn reverse_complement(&self) -> Result<Self, SeqError> {
        match self.kind {
            SeqKind::Dna => Ok(Self {
                bytes: dna::revcomp(&self.bytes),
                ..*self
            }),
            SeqKind::Rna => Ok(Self {
                bytes: rna::revcomp(&self.bytes),
                ..*self
            }),
            SeqKind::Protein => Err(SeqError::RevComp(self.kind)),
        }
    }

    pub fn convert(&self, kind: SeqKind) -> Result<Self, SeqError> {
        match (self.kind, kind) {
            (SeqKind::Dna, SeqKind::Dna) => Ok(self.clone()),
            (SeqKind::Dna, SeqKind::Rna) => Ok(Self {
                bytes: self
                    .bytes
                    .iter()
                    .map(|&b| if b == b'T' || b == b't' { b + 1 } else { b })
                    .collect(),
                kind: SeqKind::Rna,
            }),
            (SeqKind::Rna, SeqKind::Rna) => Ok(self.clone()),
            (SeqKind::Protein, SeqKind::Protein) => Ok(self.clone()),
            (from, to) => Err(SeqError::InvalidConversion(from, to)),
        }
    }
}

impl fmt::Display for SeqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeqError::InvalidConversion(from, to) => write!(f, "Cannot convert {from} to {to}")?,
            SeqError::InvalidKind(kind) => write!(f, "The provided sequence was not valid {kind}")?,
            SeqError::RevComp(kind) => write!(f, "Cannot reverse complement {kind}")?,
            SeqError::Invalid => write!(
                f,
                "The provided sequence was not valid DNA, RNA, or Protein"
            )?,
        }
        Ok(())
    }
}

impl Error for SeqError {}

impl fmt::Display for SeqKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeqKind::Dna => write!(f, "DNA")?,
            SeqKind::Rna => write!(f, "RNA")?,
            SeqKind::Protein => write!(f, "Protein")?,
        }
        Ok(())
    }
}

impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(from_utf8(&self.bytes).expect("Seq did not contain valid UTF-8"))
    }
}

// TODO: Need to add IUPAC tests for DNA, RNA, and Protein
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_dna_sequence() {
        let dna = Seq::new("AGCTTTTCATTCTGACTGCA");
        assert!(dna.is_ok());
        assert_eq!(dna.unwrap().kind(), SeqKind::Dna);
    }

    #[test]
    fn magic_rna_sequence() {
        let rna = Seq::new("AGCUUUUCAUUCUGACUGCA");
        assert!(rna.is_ok());
        assert_eq!(rna.unwrap().kind(), SeqKind::Rna);
    }

    #[test]
    fn magic_protein_sequence() {
        let protein = Seq::new("MAMAPRTEINSTRING");
        assert!(protein.is_ok());
        assert_eq!(protein.unwrap().kind(), SeqKind::Protein);
    }

    #[test]
    fn magic_not_sequence() {
        let protein = Seq::new("MAMAPUTEINSTRINX");
        assert_eq!(protein, Err(SeqError::Invalid));
    }

    #[test]
    fn read_valid_dna_sequence() {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA");
        assert!(dna.is_ok());
        assert_eq!(dna.unwrap().kind(), SeqKind::Dna);
    }

    #[test]
    fn read_invalid_dna_sequence() {
        let dna = Seq::dna("AGCTTTXCATTCTGACNGCA");
        assert_eq!(dna, Err(SeqError::InvalidKind(SeqKind::Dna)));
    }

    #[test]
    fn dna_to_string() -> Result<(), SeqError> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.to_string(), String::from("AGCTTTTCATTCTGACTGCA"));
        Ok(())
    }

    #[test]
    fn read_valid_rna_sequence() {
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA");
        assert!(rna.is_ok());
        assert_eq!(rna.unwrap().kind(), SeqKind::Rna);
    }

    #[test]
    fn read_invalid_rna_sequence() {
        let rna = Seq::rna("AGCUUTUCAUUCUGACTGCA");
        assert_eq!(rna, Err(SeqError::InvalidKind(SeqKind::Rna)));
    }

    #[test]
    fn rna_to_string() -> Result<(), SeqError> {
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA")?;
        assert_eq!(rna.to_string(), String::from("AGCUUUUCAUUCUGACUGCA"));
        Ok(())
    }

    #[test]
    fn read_valid_protein_sequence() {
        let protein = Seq::protein("MAMAPRTEINSTRING");
        assert!(protein.is_ok());
        assert_eq!(protein.unwrap().kind(), SeqKind::Protein);
    }

    #[test]
    fn read_invalid_protein_sequence() {
        let protein = Seq::protein("MAMAPUTEINSTRINX");
        assert_eq!(protein, Err(SeqError::InvalidKind(SeqKind::Protein)));
    }

    #[test]
    fn protein_to_string() -> Result<(), SeqError> {
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(protein.to_string(), String::from("MAMAPRTEINSTRING"));
        Ok(())
    }

    #[test]
    fn get_sequence_length() -> Result<(), SeqError> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.len(), 20);
        Ok(())
    }

    #[test]
    fn is_sequence_empty() -> Result<(), SeqError> {
        let dna = Seq::dna("")?;
        assert!(dna.is_empty());
        let dna = Seq::dna("ACGT")?;
        assert!(!dna.is_empty());
        Ok(())
    }

    #[test]
    fn count_nucleotides() -> Result<(), SeqError> {
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
    fn self_conversions() -> Result<(), SeqError> {
        let dna = Seq::dna("GATGGAACTTGACTACGTAAATT")?;
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA")?;
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(dna, dna.convert(SeqKind::Dna)?);
        assert_eq!(rna, rna.convert(SeqKind::Rna)?);
        assert_eq!(protein, protein.convert(SeqKind::Protein)?);
        Ok(())
    }

    #[test]
    fn dna_to_rna() -> Result<(), SeqError> {
        let dna = Seq::dna("GATGGAACTTGACTACGTAAATT")?;
        let rna = dna.convert(SeqKind::Rna)?;
        assert_eq!(rna, Seq::rna("GAUGGAACUUGACUACGUAAAUU")?);
        Ok(())
    }

    #[test]
    fn dna_to_rna_keep_case() -> Result<(), SeqError> {
        let dna = Seq::dna("GaTgGaAcTtGaCtAcGtAaAtT")?;
        let rna = dna.convert(SeqKind::Rna)?;
        assert_eq!(rna, Seq::rna("GaUgGaAcUuGaCuAcGuAaAuU")?);
        Ok(())
    }

    #[test]
    fn invalid_conversions() -> Result<(), SeqError> {
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(
            protein.convert(SeqKind::Dna),
            Err(SeqError::InvalidConversion(SeqKind::Protein, SeqKind::Dna))
        );
        assert_eq!(
            protein.convert(SeqKind::Rna),
            Err(SeqError::InvalidConversion(SeqKind::Protein, SeqKind::Rna))
        );
        Ok(())
    }

    #[test]
    fn reverse_complement_dna() -> Result<(), SeqError> {
        let dna = Seq::dna("AAAACCCGGT")?;
        assert_eq!(dna.reverse_complement()?.bytes, b"ACCGGGTTTT");
        Ok(())
    }

    #[test]
    fn reverse_complement_dna_keep_case() -> Result<(), SeqError> {
        let dna = Seq::dna("aaaacCCGGT")?;
        assert_eq!(dna.reverse_complement()?.bytes, b"ACCGGgtttt");
        Ok(())
    }

    #[test]
    fn reverse_complement_rna() -> Result<(), SeqError> {
        let rna = Seq::rna("AAAACCCGGU")?;
        assert_eq!(rna.reverse_complement()?.bytes, b"ACCGGGUUUU");
        Ok(())
    }

    #[test]
    fn reverse_complement_rna_keep_case() -> Result<(), SeqError> {
        let rna = Seq::rna("aaaacCCGGU")?;
        assert_eq!(rna.reverse_complement()?.bytes, b"ACCGGguuuu");
        Ok(())
    }

    #[test]
    fn reverse_complement_protein() -> Result<(), SeqError> {
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(
            protein.reverse_complement(),
            Err(SeqError::RevComp(SeqKind::Protein))
        );
        Ok(())
    }

    #[test]
    fn format_errors() {
        assert_eq!(
            &SeqError::InvalidConversion(SeqKind::Protein, SeqKind::Dna).to_string(),
            "Cannot convert Protein to DNA"
        );
        assert_eq!(
            &SeqError::InvalidConversion(SeqKind::Protein, SeqKind::Rna).to_string(),
            "Cannot convert Protein to RNA"
        );
        assert_eq!(
            &SeqError::InvalidKind(SeqKind::Dna).to_string(),
            "The provided sequence was not valid DNA"
        );
        assert_eq!(
            &SeqError::InvalidKind(SeqKind::Rna).to_string(),
            "The provided sequence was not valid RNA"
        );
        assert_eq!(
            &SeqError::InvalidKind(SeqKind::Protein).to_string(),
            "The provided sequence was not valid Protein"
        );

        assert_eq!(
            &SeqError::Invalid.to_string(),
            "The provided sequence was not valid DNA, RNA, or Protein"
        );

        assert_eq!(
            &SeqError::RevComp(SeqKind::Protein).to_string(),
            "Cannot reverse complement Protein"
        );
    }
}
