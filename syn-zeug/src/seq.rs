use bio::alphabets::{dna, rna};
use serde::{Deserialize, Serialize};
use std::{fmt, iter, str::from_utf8};
use strum::{EnumIter, IntoEnumIterator};

use crate::data::{ByteMap, ALPHABETS};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Error {
    InvalidConversion(Kind, Kind),
    InvalidKind(Vec<(Kind, Alphabet)>),
    RevComp(Kind),
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, EnumIter,
)]
pub enum Kind {
    Dna,
    Rna,
    Protein,
}

// TODO: Use strum to iterate over this!
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, EnumIter,
)]
pub enum Alphabet {
    Canonical, // TODO: Rename this `Default`?
    N,
    Iupac,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct Seq {
    bytes: Vec<u8>,
    kind: Kind,
    alphabet: Alphabet,
}

impl Seq {
    // TODO: Rename and clean up this rubbish...
    pub fn new_with_kind_old(
        seq: impl AsRef<[u8]>,
        kinds: impl AsRef<[Kind]>,
        alphabet: Alphabet,
    ) -> Result<Self, Error> {
        // TODO: Audit `collect` and `clone` calls
        let potential_alphabets: Vec<_> = Alphabet::iter().filter(|&a| a <= alphabet).collect();
        // TODO: Try optimising this to not zip things, just producing a Vec<(Kind, Alphabet)>
        // That would mean no unzipping on error, but also might slow things down by checking
        // ALPHABETS twice? Once for the filter contains_key and again for the actual getting
        let potential_kinds: Vec<_> = kinds
            .as_ref()
            .iter()
            .flat_map(|&k| iter::repeat(k).zip(potential_alphabets.iter().copied()))
            .filter_map(|ka| ALPHABETS.get(&ka).map(|a| (ka, a)))
            .collect();
        let seq = seq.as_ref();
        // TODO: Need to optimise this with a single-pass fold approach (instead of `find`
        // which will scan parts of the sequence several times for types like IUPAC protein)
        if let Some(&((kind, alphabet), _)) = potential_kinds.iter().find(|(_, s)| s.is_word(seq)) {
            Ok(Self {
                bytes: seq.to_vec(),
                kind,
                alphabet,
            })
        } else {
            // TODO: Is there a nicer solution using `iter::unzip`?
            Err(Error::InvalidKind(
                potential_kinds.iter().map(|&(k, _)| k).collect(),
            ))
        }
    }

    pub fn new_with_kind(
        seq: impl AsRef<[u8]>,
        kinds: impl AsRef<[Kind]>,
        alphabet: Alphabet,
    ) -> Result<Self, Error> {
        let seq = seq.as_ref();
        let potential_kinds: Vec<_> = kinds
            .as_ref()
            .iter()
            .flat_map(|&k| iter::repeat(k).zip(Alphabet::iter().filter(|&a| a <= alphabet)))
            .filter(|ka| ALPHABETS.contains_key(ka))
            .collect();
        let potential_alphabets = potential_kinds.iter().map(|ka| (ka, &ALPHABETS[ka]));

        let mut i = 0;
        for (&(kind, alphabet), a) in potential_alphabets {
            if let Some(n) = seq[i..]
                .iter()
                .position(|&c| !a.symbols.contains(c as usize))
            {
                i += n;
            } else {
                return Ok(Self {
                    bytes: seq.to_vec(),
                    kind,
                    alphabet,
                });
            }
        }
        Err(Error::InvalidKind(potential_kinds))
    }

    pub fn new_old(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind_old(&seq, Kind::iter().collect::<Vec<_>>(), Alphabet::Iupac)
    }

    pub fn new(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(&seq, Kind::iter().collect::<Vec<_>>(), Alphabet::Iupac)
    }

    // TODO: Implement `dna_n` and `dna_iupac`
    pub fn dna(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Dna], Alphabet::Canonical)
    }

    pub fn rna(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Rna], Alphabet::Canonical)
    }

    pub fn protein(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Protein], Alphabet::Canonical)
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn rev(&self) -> Self {
        Self {
            bytes: self.bytes.iter().copied().rev().collect(),
            ..*self
        }
    }

    pub fn count_elements(&self) -> ByteMap<usize> {
        let mut counts = ByteMap::default();
        for &b in &self.bytes {
            counts[b] += 1;
        }
        counts
    }

    pub fn reverse_complement(&self) -> Result<Self, Error> {
        match self.kind {
            Kind::Dna => Ok(Self {
                bytes: dna::revcomp(&self.bytes),
                ..*self
            }),
            Kind::Rna => Ok(Self {
                bytes: rna::revcomp(&self.bytes),
                ..*self
            }),
            Kind::Protein => Err(Error::RevComp(self.kind)),
        }
    }

    pub fn convert(&self, kind: Kind) -> Result<Self, Error> {
        match (self.kind, kind) {
            (from, to) if from == to => Ok(self.clone()),
            // TODO: Is this IUPAC compatible?
            (Kind::Dna, Kind::Rna) => Ok(Self {
                bytes: self
                    .bytes
                    .iter()
                    .map(|&b| if b == b'T' || b == b't' { b + 1 } else { b })
                    .collect(),
                kind: Kind::Rna,
                ..*self
            }),
            (from, to) => Err(Error::InvalidConversion(from, to)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConversion(from, to) => write!(f, "Cannot convert {from} to {to}")?,
            Error::InvalidKind(kind) => write!(f, "The provided sequence was not valid {kind:?}")?,
            Error::RevComp(kind) => write!(f, "Cannot reverse complement {kind}")?,
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Dna => write!(f, "DNA")?,
            Kind::Rna => write!(f, "RNA")?,
            Kind::Protein => write!(f, "Protein")?,
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
        assert_eq!(dna.unwrap().kind(), Kind::Dna);
    }

    #[test]
    fn magic_rna_sequence() {
        let rna = Seq::new("AGCUUUUCAUUCUGACUGCA");
        assert!(rna.is_ok());
        assert_eq!(rna.unwrap().kind(), Kind::Rna);
    }

    #[test]
    fn magic_protein_sequence() {
        let protein = Seq::new("MAMAPRTEINSTRING");
        assert!(protein.is_ok());
        assert_eq!(protein.unwrap().kind(), Kind::Protein);
    }

    #[test]
    fn magic_not_sequence() {
        let protein = Seq::new("MAMAPUTEINSTRINX");
        assert_eq!(
            protein,
            Err(Error::InvalidKind(vec![
                (Kind::Dna, Alphabet::Canonical),
                (Kind::Dna, Alphabet::N),
                (Kind::Dna, Alphabet::Iupac),
                (Kind::Rna, Alphabet::Canonical),
                (Kind::Rna, Alphabet::N),
                (Kind::Rna, Alphabet::Iupac),
                (Kind::Protein, Alphabet::Canonical),
                (Kind::Protein, Alphabet::Iupac)
            ]))
        );
    }

    #[test]
    fn read_valid_dna_sequence() {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA");
        assert!(dna.is_ok());
        assert_eq!(dna.unwrap().kind(), Kind::Dna);
    }

    #[test]
    fn read_invalid_dna_sequence() {
        let dna = Seq::dna("AGCTTTXCATTCTGACNGCA");
        assert_eq!(
            dna,
            Err(Error::InvalidKind(vec![(Kind::Dna, Alphabet::Canonical)]))
        );
    }

    #[test]
    fn dna_to_string() -> Result<(), Error> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.to_string(), String::from("AGCTTTTCATTCTGACTGCA"));
        Ok(())
    }

    #[test]
    fn read_valid_rna_sequence() {
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA");
        assert!(rna.is_ok());
        assert_eq!(rna.unwrap().kind(), Kind::Rna);
    }

    #[test]
    fn read_invalid_rna_sequence() {
        let rna = Seq::rna("AGCUUTUCAUUCUGACTGCA");
        assert_eq!(
            rna,
            Err(Error::InvalidKind(vec![(Kind::Rna, Alphabet::Canonical)]))
        );
    }

    #[test]
    fn rna_to_string() -> Result<(), Error> {
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA")?;
        assert_eq!(rna.to_string(), String::from("AGCUUUUCAUUCUGACUGCA"));
        Ok(())
    }

    #[test]
    fn read_valid_protein_sequence() {
        let protein = Seq::protein("MAMAPRTEINSTRING");
        assert!(protein.is_ok());
        assert_eq!(protein.unwrap().kind(), Kind::Protein);
    }

    #[test]
    fn read_invalid_protein_sequence() {
        let protein = Seq::protein("MAMAPUTEINSTRINX");
        assert_eq!(
            protein,
            Err(Error::InvalidKind(vec![(
                Kind::Protein,
                Alphabet::Canonical
            )]))
        );
    }

    #[test]
    fn protein_to_string() -> Result<(), Error> {
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(protein.to_string(), String::from("MAMAPRTEINSTRING"));
        Ok(())
    }

    #[test]
    fn get_sequence_length() -> Result<(), Error> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.len(), 20);
        Ok(())
    }

    #[test]
    fn is_sequence_empty() -> Result<(), Error> {
        let dna = Seq::dna("")?;
        assert!(dna.is_empty());
        let dna = Seq::dna("ACGT")?;
        assert!(!dna.is_empty());
        Ok(())
    }

    #[test]
    fn reverse_sequence() -> Result<(), Error> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        let dna_rev = Seq::dna("ACGTCAGTCTTACTTTTCGA")?;
        assert_eq!(dna.rev(), dna_rev);

        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA")?;
        let rna_rev = Seq::rna("ACGUCAGUCUUACUUUUCGA")?;
        assert_eq!(rna.rev(), rna_rev);

        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        let protein_rev = Seq::protein("GNIRTSNIETRPAMAM")?;
        assert_eq!(protein.rev(), protein_rev);
        Ok(())
    }

    #[test]
    fn count_nucleotides() -> Result<(), Error> {
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
    fn self_conversions() -> Result<(), Error> {
        let dna = Seq::dna("GATGGAACTTGACTACGTAAATT")?;
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA")?;
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(dna, dna.convert(Kind::Dna)?);
        assert_eq!(rna, rna.convert(Kind::Rna)?);
        assert_eq!(protein, protein.convert(Kind::Protein)?);
        Ok(())
    }

    #[test]
    fn dna_to_rna() -> Result<(), Error> {
        let dna = Seq::dna("GATGGAACTTGACTACGTAAATT")?;
        let rna = dna.convert(Kind::Rna)?;
        assert_eq!(rna, Seq::rna("GAUGGAACUUGACUACGUAAAUU")?);
        Ok(())
    }

    #[test]
    fn dna_to_rna_keep_case() -> Result<(), Error> {
        let dna = Seq::dna("GaTgGaAcTtGaCtAcGtAaAtT")?;
        let rna = dna.convert(Kind::Rna)?;
        assert_eq!(rna, Seq::rna("GaUgGaAcUuGaCuAcGuAaAuU")?);
        Ok(())
    }

    #[test]
    fn invalid_conversions() -> Result<(), Error> {
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(
            protein.convert(Kind::Dna),
            Err(Error::InvalidConversion(Kind::Protein, Kind::Dna))
        );
        assert_eq!(
            protein.convert(Kind::Rna),
            Err(Error::InvalidConversion(Kind::Protein, Kind::Rna))
        );
        Ok(())
    }

    #[test]
    fn reverse_complement_dna() -> Result<(), Error> {
        let dna = Seq::dna("AAAACCCGGT")?;
        assert_eq!(dna.reverse_complement()?.bytes, b"ACCGGGTTTT");
        Ok(())
    }

    #[test]
    fn reverse_complement_dna_keep_case() -> Result<(), Error> {
        let dna = Seq::dna("aaaacCCGGT")?;
        assert_eq!(dna.reverse_complement()?.bytes, b"ACCGGgtttt");
        Ok(())
    }

    #[test]
    fn reverse_complement_rna() -> Result<(), Error> {
        let rna = Seq::rna("AAAACCCGGU")?;
        assert_eq!(rna.reverse_complement()?.bytes, b"ACCGGGUUUU");
        Ok(())
    }

    #[test]
    fn reverse_complement_rna_keep_case() -> Result<(), Error> {
        let rna = Seq::rna("aaaacCCGGU")?;
        assert_eq!(rna.reverse_complement()?.bytes, b"ACCGGguuuu");
        Ok(())
    }

    #[test]
    fn reverse_complement_protein() -> Result<(), Error> {
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(
            protein.reverse_complement(),
            Err(Error::RevComp(Kind::Protein))
        );
        Ok(())
    }

    #[test]
    fn format_errors() {
        assert_eq!(
            &Error::InvalidConversion(Kind::Protein, Kind::Dna).to_string(),
            "Cannot convert Protein to DNA"
        );
        assert_eq!(
            &Error::InvalidConversion(Kind::Protein, Kind::Rna).to_string(),
            "Cannot convert Protein to RNA"
        );
        // FIXME: These need replacing!
        // assert_eq!(
        //     &Error::InvalidKind(Default::default()).to_string(),
        //     "The provided sequence was not valid DNA"
        // );
        // assert_eq!(
        //     &Error::InvalidKind(Default::default()).to_string(),
        //     "The provided sequence was not valid RNA"
        // );
        // assert_eq!(
        //     &Error::InvalidKind(Default::default()).to_string(),
        //     "The provided sequence was not valid Protein"
        // );
        assert_eq!(
            &Error::RevComp(Kind::Protein).to_string(),
            "Cannot reverse complement Protein"
        );
    }
}
