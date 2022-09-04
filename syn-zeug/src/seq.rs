use bio::{
    alphabets::{dna, rna},
    seq_analysis::orf::{self, Orf},
};
use serde::{Deserialize, Serialize};
use std::{fmt, slice::SliceIndex, str};

use crate::{
    data::{ALPHABETS, ALPHABET_MAP, CODON_TABLE, IUPAC_CODON_TABLE},
    types::{ByteMap, Case},
};

// TODO: All of the structs and impls in this file need a more logical ordering
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Error {
    InvalidConversion(Kind, Kind),
    InvalidSeq(Vec<(Kind, Alphabet)>),
    RevComp(Kind),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Kind {
    Dna,
    Rna,
    Protein,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Alphabet {
    Base,
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
    // ===== "Magic" Constructors ==================================================================

    pub fn new_with_kind(
        seq: impl AsRef<[u8]>,
        kinds: impl AsRef<[Kind]>,
        alphabet: Alphabet,
    ) -> Result<Self, Error> {
        let seq = seq.as_ref();
        let kinds = kinds.as_ref();

        let potential_kinds: Vec<_> = ALPHABETS
            .iter()
            .copied()
            .filter(|(k, a)| kinds.contains(k) && a <= &alphabet)
            .collect();
        let mut candidates: Vec<_> = potential_kinds
            .iter()
            .map(|ka| (ka, &ALPHABET_MAP[ka]))
            .collect();

        // OPTIMISATION: This algorithm will rescan large regions of sequence if the first
        // candidate fails to validate, but the best case (it is the first alphabet — or even the
        // second or third) outperforms a single-pass approach (which involves more comparisons per
        // sequence element). To ameliorate the performance hit from rescanning, the mismatch
        // character is used to filter the candidates before a rescan, ensuring that the next
        // attempted candidate won't get stuck on the same character.
        while let Some((&(kind, alphabet), a)) = candidates.get(0) {
            if let Some(c) = seq
                .iter()
                .copied()
                .find(|&c| !a.symbols.contains(c as usize))
            {
                candidates.retain(|(_, a)| a.symbols.contains(c as usize));
            } else {
                return Ok(Self {
                    bytes: seq.to_vec(),
                    kind,
                    alphabet,
                });
            }
        }

        Err(Error::InvalidSeq(potential_kinds))
    }

    pub fn new(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(&seq, [Kind::Dna, Kind::Rna, Kind::Protein], Alphabet::Iupac)
    }

    // ===== Standard Constructors =================================================================

    pub fn dna(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Dna], Alphabet::Base)
    }

    pub fn dna_n(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Dna], Alphabet::N)
    }

    pub fn dna_iupac(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Dna], Alphabet::Iupac)
    }

    pub fn rna(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Rna], Alphabet::Base)
    }

    pub fn rna_n(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Rna], Alphabet::N)
    }

    pub fn rna_iupac(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Rna], Alphabet::Iupac)
    }

    pub fn protein(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Protein], Alphabet::Base)
    }

    pub fn protein_iupac(seq: impl AsRef<[u8]>) -> Result<Self, Error> {
        Self::new_with_kind(seq, [Kind::Protein], Alphabet::Iupac)
    }

    // ===== Getters ===============================================================================

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn alphabet(&self) -> Alphabet {
        self.alphabet
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    // ===== Chainable Tools =======================================================================

    // OPTIMISATION: In the future, it may be worth looking into a `SubSeq` type that contains
    // references / slices of the original data — that would help avoid the copying and allocation
    // done by `slice::to_vec` here
    pub fn subseq(&self, range: impl SliceIndex<[u8], Output = [u8]>) -> Self {
        Self {
            bytes: self.bytes[range].to_vec(),
            kind: self.kind,
            alphabet: self.alphabet,
        }
    }

    pub fn rev(&self) -> Self {
        Self {
            bytes: self.bytes.iter().copied().rev().collect(),
            ..*self
        }
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

    pub fn normalize_case(&self, case: Case) -> Self {
        // OPTIMISATION: Pulling the `u8::to_ascii_upper/lowercase` out into a variable, then just
        // writing the whole `map` chain once seems to tank performance — likely by obscuring an
        // optimisation taking place here
        let bytes = match case {
            Case::Lower => self.bytes.iter().map(u8::to_ascii_lowercase).collect(),
            Case::Upper => self.bytes.iter().map(u8::to_ascii_uppercase).collect(),
        };

        Self { bytes, ..*self }
    }

    pub fn convert(&self, kind: Kind) -> Result<Self, Error> {
        match (self.kind, kind) {
            // OPTIMISATION: Using bytestrings and `b + 1` is 103 times faster than converting to a `str`
            // and using `str::replace`. It's also 20.4 times faster than checking case with an `else if`
            // statement and explicitly returning b'U' or b'u'
            (Kind::Dna, Kind::Rna) => Ok(Self {
                bytes: self
                    .bytes
                    .iter()
                    .map(|&b| if b == b'T' || b == b't' { b + 1 } else { b })
                    .collect(),
                kind: Kind::Rna,
                ..*self
            }),
            (Kind::Rna, Kind::Dna) => Ok(Self {
                bytes: self
                    .bytes
                    .iter()
                    .map(|&b| if b == b'U' || b == b'u' { b - 1 } else { b })
                    .collect(),
                kind: Kind::Dna,
                ..*self
            }),
            // OPTIMISATION: Lookups in the IUPAC codon table and rescanning the protein sequence
            // to tighten the alphabet both have a performance hit here. Translating IUPAC
            // nucleotides is ~0.72 times the speed of non-IUPAC sequence
            (Kind::Rna, Kind::Protein) => {
                let bytes = self.normalize_case(Case::Upper);
                let bytes = bytes.bytes.chunks_exact(3);

                let bytes = match self.alphabet {
                    Alphabet::Base => bytes.map(|c| CODON_TABLE[c]).collect(),
                    _ => bytes.map(|c| IUPAC_CODON_TABLE[c]).collect(),
                };

                match self.alphabet {
                    Alphabet::Base => Ok(Self {
                        bytes,
                        kind: Kind::Protein,
                        ..*self
                    }),
                    _ => Seq::protein_iupac(bytes),
                }
            }
            (Kind::Dna, Kind::Protein) => self.convert(Kind::Rna)?.convert(Kind::Protein),
            (from, to) if from == to => Ok(self.clone()),
            (from, to) => Err(Error::InvalidConversion(from, to)),
        }
    }

    // TODO: Add parameters allowing the user to change which start and stop codons are used
    // TODO: Needs benchmarking!
    // TODO: Make this IUPAC aware?
    // TODO: Write a function that allows to sequence to be compared, taking into account the IUPAC
    // ambiguities!
    // TODO: Codon table as nested match statements picking codons apart one base at a time?
    // TODO: Write a function that takes a list of sequences, finds which bases are present in each
    // position, unifies them into a few sequences with IUPAC placeholders, then expands the set to
    // also include all IUPAC supersets. Takes a set of Base sequences, and adds to that set all
    // IUPAC variations that are guaranteed to, no matter how their ambiguities are resolved, be an
    // original sequence from the Base set. Would make a nice const fn!
    // FIXME: Needs to check sequence kind
    pub fn find_orfs(&self, min_len: usize) -> Vec<(Orf, Self)> {
        let start_codons = vec![b"ATG"];
        // TODO: Only use IUPAC if input is IUPAC?
        let stop_codons = vec![b"TGA", b"TAG", b"TAA", b"TAR", b"TRA"];
        let min_len = min_len * 3;
        // TODO: `orf::Finder::new()` should be changed to take `AsRef<[&[u8; 3]]>`
        let finder = orf::Finder::new(start_codons, stop_codons, min_len);

        finder
            .find_all(&self.bytes)
            .map(|orf| (orf, self.subseq(orf.start..orf.end)))
            .collect()
    }

    // ===== Terminal Tools ========================================================================

    // OPTIMISATION: This code indexing a sparse ByteMap to keep counts is 14.3 times faster than
    // the equivalent (and more canonical) code written using `HashMap` and the `Entry` API
    pub fn count_elements(&self) -> ByteMap<usize> {
        let mut counts = ByteMap::default();
        for &b in &self.bytes {
            counts[b] += 1;
        }
        counts
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConversion(from, to) => write!(f, "Cannot convert {from} to {to}")?,
            Error::InvalidSeq(kinds) => {
                let kinds: Vec<_> = kinds.iter().map(|(k, a)| format!("{k} ({a})")).collect();
                let kinds = kinds[..kinds.len() - 1].join(", ") + ", or " + &kinds[kinds.len() - 1];
                write!(f, "The provided sequence was not valid {kinds}")?;
            }
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

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Alphabet::Base => write!(f, "Base")?,
            Alphabet::N => write!(f, "N")?,
            Alphabet::Iupac => write!(f, "IUPAC")?,
        }
        Ok(())
    }
}

impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(str::from_utf8(&self.bytes).expect("Seq did not contain valid UTF-8"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== "Magic" Sequence Constructor Tests (Typical Usage) ====================================

    #[test]
    fn magic_dna_sequence() {
        let dna = Seq::new("AGCTTTTCATTCTGACTGCA");
        let dna = dna.unwrap();
        assert_eq!(dna.kind(), Kind::Dna);
        assert_eq!(dna.alphabet(), Alphabet::Base);
    }

    #[test]
    fn magic_dna_n_sequence() {
        let dna = Seq::new("AGCTTNTCATTCTNNCTGCA");
        let dna = dna.unwrap();
        assert_eq!(dna.kind(), Kind::Dna);
        assert_eq!(dna.alphabet(), Alphabet::N);
    }

    #[test]
    fn magic_dna_iupac_sequence() {
        let dna = Seq::new("ABCTTNTCASTCTNNCTGWA");
        let dna = dna.unwrap();
        assert_eq!(dna.kind(), Kind::Dna);
        assert_eq!(dna.alphabet(), Alphabet::Iupac);
    }

    #[test]
    fn magic_rna_sequence() {
        let rna = Seq::new("AGCUUUUCAUUCUGACUGCA");
        let rna = rna.unwrap();
        assert_eq!(rna.kind(), Kind::Rna);
        assert_eq!(rna.alphabet(), Alphabet::Base);
    }

    #[test]
    fn magic_rna_n_sequence() {
        let rna = Seq::new("AGCUNNUCAUUCUGANUGCA");
        let rna = rna.unwrap();
        assert_eq!(rna.kind(), Kind::Rna);
        assert_eq!(rna.alphabet(), Alphabet::N);
    }

    #[test]
    fn magic_rna_iupac_sequence() {
        let rna = Seq::new("ADHUNNUCAUUVUGANUKCA");
        let rna = rna.unwrap();
        assert_eq!(rna.kind(), Kind::Rna);
        assert_eq!(rna.alphabet(), Alphabet::Iupac);
    }

    #[test]
    fn magic_protein_sequence() {
        let protein = Seq::new("MAMAPRTEINSTRING");
        let protein = protein.unwrap();
        assert_eq!(protein.kind(), Kind::Protein);
        assert_eq!(protein.alphabet(), Alphabet::Base);
    }

    #[test]
    fn magic_protein_iupac_sequence() {
        let protein = Seq::new("MAMXPRTEIBSTRINZ");
        let protein = protein.unwrap();
        assert_eq!(protein.kind(), Kind::Protein);
        assert_eq!(protein.alphabet(), Alphabet::Iupac);
    }

    // ===== "Magic" Sequence Constructor Tests (Edge Cases + Errors) ==============================

    #[test]
    fn magic_tricky_rna_iupac_sequence() {
        let rna = Seq::new("ADHANNCCAGGVAGANCKCAU");
        let rna = rna.unwrap();
        assert_eq!(rna.kind(), Kind::Rna);
        assert_eq!(rna.alphabet(), Alphabet::Iupac);
    }

    #[test]
    fn magic_tricky_rna_sequence() {
        let rna = Seq::new("AGCTTTTCATTCTGACTGCAU");
        assert!(rna.is_err());
    }

    #[test]
    fn magic_not_sequence() {
        let protein = Seq::new("MAMAPUTEINSTRINX");
        assert_eq!(
            protein,
            Err(Error::InvalidSeq(vec![
                (Kind::Dna, Alphabet::Base),
                (Kind::Rna, Alphabet::Base),
                (Kind::Dna, Alphabet::N),
                (Kind::Rna, Alphabet::N),
                (Kind::Protein, Alphabet::Base),
                (Kind::Dna, Alphabet::Iupac),
                (Kind::Rna, Alphabet::Iupac),
                (Kind::Protein, Alphabet::Iupac)
            ]))
        );
    }

    #[test]
    fn magic_all_alphabets_reachable() {
        // NOTE: The priority list of alphabets that the magic constructor searches through must be
        // ordered in a way that makes it possible (with the right input) to reach all of the
        // `Kind` + `Alphabet` combinations. This test is here so that, even if the order in which
        // alphabets are tried changes in the future, this property holds
        for (i, curr) in ALPHABETS.iter().enumerate() {
            assert!(&ALPHABETS[..i].iter().all(|past| {
                let past = &ALPHABET_MAP[past].symbols;
                let curr = &ALPHABET_MAP[curr].symbols;
                !past.is_superset(curr)
            }));
        }
    }

    // ===== Standard Sequence Constructor Tests (DNA) =============================================

    #[test]
    fn read_valid_dna_sequence() {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA");
        let dna = dna.unwrap();
        assert_eq!(dna.kind(), Kind::Dna);
        assert_eq!(dna.alphabet(), Alphabet::Base);
    }

    #[test]
    fn read_invalid_dna_sequence() {
        let dna = Seq::dna("AGCTTTXCATTCTGACNGCA");
        assert_eq!(
            dna,
            Err(Error::InvalidSeq(vec![(Kind::Dna, Alphabet::Base)]))
        );
    }

    #[test]
    fn read_valid_dna_n_sequence() {
        let dna = Seq::dna_n("AGCTTNTCATTCTNNCTGCA");
        let dna = dna.unwrap();
        assert_eq!(dna.kind(), Kind::Dna);
        assert_eq!(dna.alphabet(), Alphabet::N);
    }

    #[test]
    fn read_invalid_dna_n_sequence() {
        let dna = Seq::dna_n("AGCUUNUCAUUCUNNCUGCA");
        assert_eq!(
            dna,
            Err(Error::InvalidSeq(vec![
                (Kind::Dna, Alphabet::Base),
                (Kind::Dna, Alphabet::N)
            ]))
        );
    }

    #[test]
    fn read_valid_dna_iupac_sequence() {
        let dna = Seq::dna_iupac("ABCTTNTCASTCTNNCTGWA");
        let dna = dna.unwrap();
        assert_eq!(dna.kind(), Kind::Dna);
        assert_eq!(dna.alphabet(), Alphabet::Iupac);
    }

    #[test]
    fn read_invalid_dna_iupac_sequence() {
        let dna = Seq::dna_iupac("ABCUUNUCASUCUNNCUGWA");
        assert_eq!(
            dna,
            Err(Error::InvalidSeq(vec![
                (Kind::Dna, Alphabet::Base),
                (Kind::Dna, Alphabet::N),
                (Kind::Dna, Alphabet::Iupac)
            ]))
        );
    }

    #[test]
    fn read_valid_dna_downcasting() {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA");
        let dna_n = Seq::dna_n("AGCTTTTCATTCTGACTGCA");
        let dna_iupac = Seq::dna_iupac("AGCTTTTCATTCTGACTGCA");
        assert_eq!(dna, dna_n);
        assert_eq!(dna_n, dna_iupac);
    }

    // ===== Standard Sequence Constructor Tests (RNA) =============================================

    #[test]
    fn read_valid_rna_sequence() {
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA");
        let rna = rna.unwrap();
        assert_eq!(rna.kind(), Kind::Rna);
        assert_eq!(rna.alphabet(), Alphabet::Base);
    }

    #[test]
    fn read_invalid_rna_sequence() {
        let rna = Seq::rna("AGCUUTUCAUUCUGACTGCA");
        assert_eq!(
            rna,
            Err(Error::InvalidSeq(vec![(Kind::Rna, Alphabet::Base)]))
        );
    }

    #[test]
    fn read_valid_rna_n_sequence() {
        let rna = Seq::rna_n("AGCUUNUCAUUCUNNCUGCA");
        let rna = rna.unwrap();
        assert_eq!(rna.kind(), Kind::Rna);
        assert_eq!(rna.alphabet(), Alphabet::N);
    }

    #[test]
    fn read_invalid_rna_n_sequence() {
        let rna = Seq::rna_n("AGCTTNTCATTCTNNCTGCA");
        assert_eq!(
            rna,
            Err(Error::InvalidSeq(vec![
                (Kind::Rna, Alphabet::Base),
                (Kind::Rna, Alphabet::N)
            ]))
        );
    }

    #[test]
    fn read_valid_rna_iupac_sequence() {
        let rna = Seq::rna_iupac("ABCUUNUCASUCUNNCUGWA");
        let rna = rna.unwrap();
        assert_eq!(rna.kind(), Kind::Rna);
        assert_eq!(rna.alphabet(), Alphabet::Iupac);
    }

    #[test]
    fn read_invalid_rna_iupac_sequence() {
        let rna = Seq::rna_iupac("ABCTTNTCASTCTNNCTGWA");
        assert_eq!(
            rna,
            Err(Error::InvalidSeq(vec![
                (Kind::Rna, Alphabet::Base),
                (Kind::Rna, Alphabet::N),
                (Kind::Rna, Alphabet::Iupac)
            ]))
        );
    }

    #[test]
    fn read_valid_rna_downcasting() {
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA");
        let rna_n = Seq::rna_n("AGCUUUUCAUUCUGACUGCA");
        let rna_iupac = Seq::rna_iupac("AGCUUUUCAUUCUGACUGCA");
        assert_eq!(rna, rna_n);
        assert_eq!(rna_n, rna_iupac);
    }

    // ===== Standard Sequence Constructor Tests (Protein) =========================================

    #[test]
    fn read_valid_protein_sequence() {
        let protein = Seq::protein("MAMAPRTEINSTRING");
        let protein = protein.unwrap();
        assert_eq!(protein.kind(), Kind::Protein);
        assert_eq!(protein.alphabet(), Alphabet::Base);
    }

    #[test]
    fn read_invalid_protein_sequence() {
        let protein = Seq::protein("MAMAPUTEINSTRINX");
        assert_eq!(
            protein,
            Err(Error::InvalidSeq(vec![(Kind::Protein, Alphabet::Base)]))
        );
    }

    #[test]
    fn read_valid_protein_iupac_sequence() {
        let protein_iupac = Seq::protein_iupac("MAMXPRTEIBSTRINZ");
        let protein_iupac = protein_iupac.unwrap();
        assert_eq!(protein_iupac.kind(), Kind::Protein);
        assert_eq!(protein_iupac.alphabet(), Alphabet::Iupac);
    }

    #[test]
    fn read_invalid_protein_iupac_sequence() {
        let protein_iupac = Seq::protein_iupac("MAMXPUTEIBSTRINZ");
        assert_eq!(
            protein_iupac,
            Err(Error::InvalidSeq(vec![
                (Kind::Protein, Alphabet::Base),
                (Kind::Protein, Alphabet::Iupac)
            ]))
        );
    }

    // ===== Case Conversion Tests =================================================================

    #[test]
    fn to_lowercase() -> Result<(), Error> {
        let dna = Seq::dna("GaTgGaAcTtGaCtAcGtAaAtT")?;
        assert_eq!(
            dna.normalize_case(Case::Lower),
            Seq::dna("gatggaacttgactacgtaaatt")?
        );
        Ok(())
    }

    #[test]
    fn to_uppercase() -> Result<(), Error> {
        let dna = Seq::dna("GaTgGaAcTtGaCtAcGtAaAtT")?;
        assert_eq!(
            dna.normalize_case(Case::Upper),
            Seq::dna("GATGGAACTTGACTACGTAAATT")?
        );
        Ok(())
    }

    #[test]
    fn to_lowercase_protein() -> Result<(), Error> {
        let protein = Seq::protein("MAMAPRTEINSTRING*")?;
        assert_eq!(
            protein.normalize_case(Case::Lower),
            Seq::protein("mamaprteinstring*")?
        );
        Ok(())
    }

    // ===== String Conversion Tests ===============================================================

    #[test]
    fn dna_to_string() -> Result<(), Error> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.to_string(), String::from("AGCTTTTCATTCTGACTGCA"));
        Ok(())
    }

    #[test]
    fn rna_to_string() -> Result<(), Error> {
        let rna = Seq::rna("AGCUUUUCAUUCUGACUGCA")?;
        assert_eq!(rna.to_string(), String::from("AGCUUUUCAUUCUGACUGCA"));
        Ok(())
    }

    #[test]
    fn protein_to_string() -> Result<(), Error> {
        let protein = Seq::protein("MAMAPRTEINSTRING")?;
        assert_eq!(protein.to_string(), String::from("MAMAPRTEINSTRING"));
        Ok(())
    }

    // ===== Sequence Length Tests =================================================================

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

    // ===== Subsequence Tests =====================================================================

    #[test]
    fn get_subseq_length() -> Result<(), Error> {
        let dna = Seq::dna("AGCTTTTCATTCTGACTGCA")?;
        assert_eq!(dna.len(), 20);
        let dna = dna.subseq(..);
        assert_eq!(dna.len(), 20);
        let dna = dna.subseq(5..);
        assert_eq!(dna.len(), 15);
        let dna = dna.subseq(5..10);
        assert_eq!(dna.len(), 5);
        assert_eq!(dna.to_string(), "TCTGA");
        Ok(())
    }

    #[test]
    fn preserve_subseq_metadata() -> Result<(), Error> {
        let seq = Seq::new("AGCTTTTCATTCTGACTGCS")?;
        assert_eq!(seq.kind(), Kind::Protein);
        assert_eq!(seq.alphabet(), Alphabet::Base);
        let seq = seq.subseq(..10);
        assert_eq!(seq.len(), 10);
        assert_eq!(seq.to_string(), "AGCTTTTCAT");
        assert_eq!(seq.kind(), Kind::Protein);
        assert_eq!(seq.alphabet(), Alphabet::Base);
        let seq = Seq::new("AGCTTTTCAT")?;
        assert_eq!(seq.kind(), Kind::Dna);
        assert_eq!(seq.alphabet(), Alphabet::Base);
        Ok(())
    }

    // ===== Sequence Reversal Tool Tests ==========================================================

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

    // ===== Element Counting Tool Tests ===========================================================

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

    // ===== ORF Finding + Translation Tool Tests ==================================================

    #[test]
    fn find_orfs() -> Result<(), Error> {
        let dna = Seq::dna(
            "AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGA\
             TTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG",
        )?;
        assert_eq!(
            vec![
                (
                    Orf {
                        start: 4,
                        end: 10,
                        offset: 1,
                    },
                    Seq::dna("ATGTAG")?
                ),
                (
                    Orf {
                        start: 24,
                        end: 69,
                        offset: 0,
                    },
                    Seq::dna("ATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAA")?
                ),
                (
                    Orf {
                        start: 30,
                        end: 69,
                        offset: 0,
                    },
                    Seq::dna("ATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAA")?
                )
            ],
            dna.find_orfs(1)
        );
        assert_eq!(
            vec![
                (
                    Orf {
                        start: 70,
                        end: 76,
                        offset: 1,
                    },
                    Seq::dna("ATGTAA")?
                ),
                (
                    Orf {
                        start: 5,
                        end: 86,
                        offset: 2,
                    },
                    Seq::dna("ATGCTACTCGGATCATTCAGGCTTATTCCAAAAGAGACTCTAATCCAAGTCGCGGGGTCATCCCCATGTAACCTGAGTTAG")?
                ),
            ],
            dna.reverse_complement()?.find_orfs(1)
        );
        Ok(())
    }

    #[test]
    fn find_large_orfs() -> Result<(), Error> {
        let dna = Seq::dna(
            "AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGA\
             TTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG",
        )?;
        assert_eq!(
            vec![
                (
                    Orf {
                        start: 24,
                        end: 69,
                        offset: 0,
                    },
                    Seq::dna("ATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAA")?
                ),
                (
                    Orf {
                        start: 30,
                        end: 69,
                        offset: 0,
                    },
                    Seq::dna("ATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAA")?
                )
            ],
            dna.find_orfs(12)
        );
        assert_eq!(
            vec![
                (
                    Orf {
                        start: 5,
                        end: 86,
                        offset: 2,
                    },
                    Seq::dna("ATGCTACTCGGATCATTCAGGCTTATTCCAAAAGAGACTCTAATCCAAGTCGCGGGGTCATCCCCATGTAACCTGAGTTAG")?
                ),
            ],
            dna.reverse_complement()?.find_orfs(12)
        );
        Ok(())
    }

    // ===== Sequence Conversion Tool Tests ========================================================

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
    fn rna_to_dna() -> Result<(), Error> {
        let rna = Seq::rna("GAUGGAACUUGACUACGUAAAUU")?;
        let dna = rna.convert(Kind::Dna)?;
        assert_eq!(dna, Seq::dna("GATGGAACTTGACTACGTAAATT")?);
        Ok(())
    }

    #[test]
    fn rna_to_dna_keep_case() -> Result<(), Error> {
        let rna = Seq::rna("GaUgGaAcUuGaCuAcGuAaAuU")?;
        let dna = rna.convert(Kind::Dna)?;
        assert_eq!(dna, Seq::dna("GaTgGaAcTtGaCtAcGtAaAtT")?);
        Ok(())
    }

    #[test]
    fn rna_to_protein() -> Result<(), Error> {
        let rna = Seq::rna("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA")?;
        let protein = rna.convert(Kind::Protein)?;
        assert_eq!(protein, Seq::protein("MAMAPRTEINSTRING*")?);
        Ok(())
    }

    #[test]
    fn rna_all_codons() -> Result<(), Error> {
        let rna = Seq::rna(
            "UUUUUCUUAUUGUCUUCCUCAUCGUAUUACUAAUAGUGUUGCUGAUGGCUUCUCCUACUGCCUCCCCCACCGCAUCACCAACAGCG\
             UCGCCGACGGAUUAUCAUAAUGACUACCACAACGAAUAACAAAAAGAGUAGCAGAAGGGUUGUCGUAGUGGCUGCCGCAGCGGAUG\
             ACGAAGAGGGUGGCGGAGGG",
        )?;
        let protein = rna.convert(Kind::Protein)?;
        assert_eq!(
            protein,
            Seq::protein("FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG")?
        );
        Ok(())
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn rna_all_iupac_codons() -> Result<(), Error> {
        let rna = Seq::rna_iupac(
            "AAAAABAACAADAAGAAHAAKAAMAANAARAASAAUAAVAAWAAYABAABBABCABDABGABHABKABMABNABRABSABUABVAB\
             WABYACAACBACCACDACGACHACKACMACNACRACSACUACVACWACYADAADBADCADDADGADHADKADMADNADRADSADUA\
             DVADWADYAGAAGBAGCAGDAGGAGHAGKAGMAGNAGRAGSAGUAGVAGWAGYAHAAHBAHCAHDAHGAHHAHKAHMAHNAHRAHS\
             AHUAHVAHWAHYAKAAKBAKCAKDAKGAKHAKKAKMAKNAKRAKSAKUAKVAKWAKYAMAAMBAMCAMDAMGAMHAMKAMMAMNAM\
             RAMSAMUAMVAMWAMYANAANBANCANDANGANHANKANMANNANRANSANUANVANWANYARAARBARCARDARGARHARKARMA\
             RNARRARSARUARVARWARYASAASBASCASDASGASHASKASMASNASRASSASUASVASWASYAUAAUBAUCAUDAUGAUHAUK\
             AUMAUNAURAUSAUUAUVAUWAUYAVAAVBAVCAVDAVGAVHAVKAVMAVNAVRAVSAVUAVVAVWAVYAWAAWBAWCAWDAWGAW\
             HAWKAWMAWNAWRAWSAWUAWVAWWAWYAYAAYBAYCAYDAYGAYHAYKAYMAYNAYRAYSAYUAYVAYWAYYBAABABBACBADB\
             AGBAHBAKBAMBANBARBASBAUBAVBAWBAYBBABBBBBCBBDBBGBBHBBKBBMBBNBBRBBSBBUBBVBBWBBYBCABCBBCC\
             BCDBCGBCHBCKBCMBCNBCRBCSBCUBCVBCWBCYBDABDBBDCBDDBDGBDHBDKBDMBDNBDRBDSBDUBDVBDWBDYBGABG\
             BBGCBGDBGGBGHBGKBGMBGNBGRBGSBGUBGVBGWBGYBHABHBBHCBHDBHGBHHBHKBHMBHNBHRBHSBHUBHVBHWBHYB\
             KABKBBKCBKDBKGBKHBKKBKMBKNBKRBKSBKUBKVBKWBKYBMABMBBMCBMDBMGBMHBMKBMMBMNBMRBMSBMUBMVBMW\
             BMYBNABNBBNCBNDBNGBNHBNKBNMBNNBNRBNSBNUBNVBNWBNYBRABRBBRCBRDBRGBRHBRKBRMBRNBRRBRSBRUBR\
             VBRWBRYBSABSBBSCBSDBSGBSHBSKBSMBSNBSRBSSBSUBSVBSWBSYBUABUBBUCBUDBUGBUHBUKBUMBUNBURBUSB\
             UUBUVBUWBUYBVABVBBVCBVDBVGBVHBVKBVMBVNBVRBVSBVUBVVBVWBVYBWABWBBWCBWDBWGBWHBWKBWMBWNBWR\
             BWSBWUBWVBWWBWYBYABYBBYCBYDBYGBYHBYKBYMBYNBYRBYSBYUBYVBYWBYYCAACABCACCADCAGCAHCAKCAMCA\
             NCARCASCAUCAVCAWCAYCBACBBCBCCBDCBGCBHCBKCBMCBNCBRCBSCBUCBVCBWCBYCCACCBCCCCCDCCGCCHCCKC\
             CMCCNCCRCCSCCUCCVCCWCCYCDACDBCDCCDDCDGCDHCDKCDMCDNCDRCDSCDUCDVCDWCDYCGACGBCGCCGDCGGCGH\
             CGKCGMCGNCGRCGSCGUCGVCGWCGYCHACHBCHCCHDCHGCHHCHKCHMCHNCHRCHSCHUCHVCHWCHYCKACKBCKCCKDCK\
             GCKHCKKCKMCKNCKRCKSCKUCKVCKWCKYCMACMBCMCCMDCMGCMHCMKCMMCMNCMRCMSCMUCMVCMWCMYCNACNBCNCC\
             NDCNGCNHCNKCNMCNNCNRCNSCNUCNVCNWCNYCRACRBCRCCRDCRGCRHCRKCRMCRNCRRCRSCRUCRVCRWCRYCSACSB\
             CSCCSDCSGCSHCSKCSMCSNCSRCSSCSUCSVCSWCSYCUACUBCUCCUDCUGCUHCUKCUMCUNCURCUSCUUCUVCUWCUYCV\
             ACVBCVCCVDCVGCVHCVKCVMCVNCVRCVSCVUCVVCVWCVYCWACWBCWCCWDCWGCWHCWKCWMCWNCWRCWSCWUCWVCWWC\
             WYCYACYBCYCCYDCYGCYHCYKCYMCYNCYRCYSCYUCYVCYWCYYDAADABDACDADDAGDAHDAKDAMDANDARDASDAUDAV\
             DAWDAYDBADBBDBCDBDDBGDBHDBKDBMDBNDBRDBSDBUDBVDBWDBYDCADCBDCCDCDDCGDCHDCKDCMDCNDCRDCSDC\
             UDCVDCWDCYDDADDBDDCDDDDDGDDHDDKDDMDDNDDRDDSDDUDDVDDWDDYDGADGBDGCDGDDGGDGHDGKDGMDGNDGRD\
             GSDGUDGVDGWDGYDHADHBDHCDHDDHGDHHDHKDHMDHNDHRDHSDHUDHVDHWDHYDKADKBDKCDKDDKGDKHDKKDKMDKN\
             DKRDKSDKUDKVDKWDKYDMADMBDMCDMDDMGDMHDMKDMMDMNDMRDMSDMUDMVDMWDMYDNADNBDNCDNDDNGDNHDNKDN\
             MDNNDNRDNSDNUDNVDNWDNYDRADRBDRCDRDDRGDRHDRKDRMDRNDRRDRSDRUDRVDRWDRYDSADSBDSCDSDDSGDSHD\
             SKDSMDSNDSRDSSDSUDSVDSWDSYDUADUBDUCDUDDUGDUHDUKDUMDUNDURDUSDUUDUVDUWDUYDVADVBDVCDVDDVG\
             DVHDVKDVMDVNDVRDVSDVUDVVDVWDVYDWADWBDWCDWDDWGDWHDWKDWMDWNDWRDWSDWUDWVDWWDWYDYADYBDYCDY\
             DDYGDYHDYKDYMDYNDYRDYSDYUDYVDYWDYYGAAGABGACGADGAGGAHGAKGAMGANGARGASGAUGAVGAWGAYGBAGBBG\
             BCGBDGBGGBHGBKGBMGBNGBRGBSGBUGBVGBWGBYGCAGCBGCCGCDGCGGCHGCKGCMGCNGCRGCSGCUGCVGCWGCYGDA\
             GDBGDCGDDGDGGDHGDKGDMGDNGDRGDSGDUGDVGDWGDYGGAGGBGGCGGDGGGGGHGGKGGMGGNGGRGGSGGUGGVGGWGG\
             YGHAGHBGHCGHDGHGGHHGHKGHMGHNGHRGHSGHUGHVGHWGHYGKAGKBGKCGKDGKGGKHGKKGKMGKNGKRGKSGKUGKVG\
             KWGKYGMAGMBGMCGMDGMGGMHGMKGMMGMNGMRGMSGMUGMVGMWGMYGNAGNBGNCGNDGNGGNHGNKGNMGNNGNRGNSGNU\
             GNVGNWGNYGRAGRBGRCGRDGRGGRHGRKGRMGRNGRRGRSGRUGRVGRWGRYGSAGSBGSCGSDGSGGSHGSKGSMGSNGSRGS\
             SGSUGSVGSWGSYGUAGUBGUCGUDGUGGUHGUKGUMGUNGURGUSGUUGUVGUWGUYGVAGVBGVCGVDGVGGVHGVKGVMGVNG\
             VRGVSGVUGVVGVWGVYGWAGWBGWCGWDGWGGWHGWKGWMGWNGWRGWSGWUGWVGWWGWYGYAGYBGYCGYDGYGGYHGYKGYM\
             GYNGYRGYSGYUGYVGYWGYYHAAHABHACHADHAGHAHHAKHAMHANHARHASHAUHAVHAWHAYHBAHBBHBCHBDHBGHBHHB\
             KHBMHBNHBRHBSHBUHBVHBWHBYHCAHCBHCCHCDHCGHCHHCKHCMHCNHCRHCSHCUHCVHCWHCYHDAHDBHDCHDDHDGH\
             DHHDKHDMHDNHDRHDSHDUHDVHDWHDYHGAHGBHGCHGDHGGHGHHGKHGMHGNHGRHGSHGUHGVHGWHGYHHAHHBHHCHHD\
             HHGHHHHHKHHMHHNHHRHHSHHUHHVHHWHHYHKAHKBHKCHKDHKGHKHHKKHKMHKNHKRHKSHKUHKVHKWHKYHMAHMBHM\
             CHMDHMGHMHHMKHMMHMNHMRHMSHMUHMVHMWHMYHNAHNBHNCHNDHNGHNHHNKHNMHNNHNRHNSHNUHNVHNWHNYHRAH\
             RBHRCHRDHRGHRHHRKHRMHRNHRRHRSHRUHRVHRWHRYHSAHSBHSCHSDHSGHSHHSKHSMHSNHSRHSSHSUHSVHSWHSY\
             HUAHUBHUCHUDHUGHUHHUKHUMHUNHURHUSHUUHUVHUWHUYHVAHVBHVCHVDHVGHVHHVKHVMHVNHVRHVSHVUHVVHV\
             WHVYHWAHWBHWCHWDHWGHWHHWKHWMHWNHWRHWSHWUHWVHWWHWYHYAHYBHYCHYDHYGHYHHYKHYMHYNHYRHYSHYUH\
             YVHYWHYYKAAKABKACKADKAGKAHKAKKAMKANKARKASKAUKAVKAWKAYKBAKBBKBCKBDKBGKBHKBKKBMKBNKBRKBS\
             KBUKBVKBWKBYKCAKCBKCCKCDKCGKCHKCKKCMKCNKCRKCSKCUKCVKCWKCYKDAKDBKDCKDDKDGKDHKDKKDMKDNKD\
             RKDSKDUKDVKDWKDYKGAKGBKGCKGDKGGKGHKGKKGMKGNKGRKGSKGUKGVKGWKGYKHAKHBKHCKHDKHGKHHKHKKHMK\
             HNKHRKHSKHUKHVKHWKHYKKAKKBKKCKKDKKGKKHKKKKKMKKNKKRKKSKKUKKVKKWKKYKMAKMBKMCKMDKMGKMHKMK\
             KMMKMNKMRKMSKMUKMVKMWKMYKNAKNBKNCKNDKNGKNHKNKKNMKNNKNRKNSKNUKNVKNWKNYKRAKRBKRCKRDKRGKR\
             HKRKKRMKRNKRRKRSKRUKRVKRWKRYKSAKSBKSCKSDKSGKSHKSKKSMKSNKSRKSSKSUKSVKSWKSYKUAKUBKUCKUDK\
             UGKUHKUKKUMKUNKURKUSKUUKUVKUWKUYKVAKVBKVCKVDKVGKVHKVKKVMKVNKVRKVSKVUKVVKVWKVYKWAKWBKWC\
             KWDKWGKWHKWKKWMKWNKWRKWSKWUKWVKWWKWYKYAKYBKYCKYDKYGKYHKYKKYMKYNKYRKYSKYUKYVKYWKYYMAAMA\
             BMACMADMAGMAHMAKMAMMANMARMASMAUMAVMAWMAYMBAMBBMBCMBDMBGMBHMBKMBMMBNMBRMBSMBUMBVMBWMBYM\
             CAMCBMCCMCDMCGMCHMCKMCMMCNMCRMCSMCUMCVMCWMCYMDAMDBMDCMDDMDGMDHMDKMDMMDNMDRMDSMDUMDVMDW\
             MDYMGAMGBMGCMGDMGGMGHMGKMGMMGNMGRMGSMGUMGVMGWMGYMHAMHBMHCMHDMHGMHHMHKMHMMHNMHRMHSMHUMH\
             VMHWMHYMKAMKBMKCMKDMKGMKHMKKMKMMKNMKRMKSMKUMKVMKWMKYMMAMMBMMCMMDMMGMMHMMKMMMMMNMMRMMSM\
             MUMMVMMWMMYMNAMNBMNCMNDMNGMNHMNKMNMMNNMNRMNSMNUMNVMNWMNYMRAMRBMRCMRDMRGMRHMRKMRMMRNMRR\
             MRSMRUMRVMRWMRYMSAMSBMSCMSDMSGMSHMSKMSMMSNMSRMSSMSUMSVMSWMSYMUAMUBMUCMUDMUGMUHMUKMUMMU\
             NMURMUSMUUMUVMUWMUYMVAMVBMVCMVDMVGMVHMVKMVMMVNMVRMVSMVUMVVMVWMVYMWAMWBMWCMWDMWGMWHMWKM\
             WMMWNMWRMWSMWUMWVMWWMWYMYAMYBMYCMYDMYGMYHMYKMYMMYNMYRMYSMYUMYVMYWMYYNAANABNACNADNAGNAH\
             NAKNAMNANNARNASNAUNAVNAWNAYNBANBBNBCNBDNBGNBHNBKNBMNBNNBRNBSNBUNBVNBWNBYNCANCBNCCNCDNC\
             GNCHNCKNCMNCNNCRNCSNCUNCVNCWNCYNDANDBNDCNDDNDGNDHNDKNDMNDNNDRNDSNDUNDVNDWNDYNGANGBNGCN\
             GDNGGNGHNGKNGMNGNNGRNGSNGUNGVNGWNGYNHANHBNHCNHDNHGNHHNHKNHMNHNNHRNHSNHUNHVNHWNHYNKANKB\
             NKCNKDNKGNKHNKKNKMNKNNKRNKSNKUNKVNKWNKYNMANMBNMCNMDNMGNMHNMKNMMNMNNMRNMSNMUNMVNMWNMYNN\
             ANNBNNCNNDNNGNNHNNKNNMNNNNNRNNSNNUNNVNNWNNYNRANRBNRCNRDNRGNRHNRKNRMNRNNRRNRSNRUNRVNRWN\
             RYNSANSBNSCNSDNSGNSHNSKNSMNSNNSRNSSNSUNSVNSWNSYNUANUBNUCNUDNUGNUHNUKNUMNUNNURNUSNUUNUV\
             NUWNUYNVANVBNVCNVDNVGNVHNVKNVMNVNNVRNVSNVUNVVNVWNVYNWANWBNWCNWDNWGNWHNWKNWMNWNNWRNWSNW\
             UNWVNWWNWYNYANYBNYCNYDNYGNYHNYKNYMNYNNYRNYSNYUNYVNYWNYYRAARABRACRADRAGRAHRAKRAMRANRARR\
             ASRAURAVRAWRAYRBARBBRBCRBDRBGRBHRBKRBMRBNRBRRBSRBURBVRBWRBYRCARCBRCCRCDRCGRCHRCKRCMRCN\
             RCRRCSRCURCVRCWRCYRDARDBRDCRDDRDGRDHRDKRDMRDNRDRRDSRDURDVRDWRDYRGARGBRGCRGDRGGRGHRGKRG\
             MRGNRGRRGSRGURGVRGWRGYRHARHBRHCRHDRHGRHHRHKRHMRHNRHRRHSRHURHVRHWRHYRKARKBRKCRKDRKGRKHR\
             KKRKMRKNRKRRKSRKURKVRKWRKYRMARMBRMCRMDRMGRMHRMKRMMRMNRMRRMSRMURMVRMWRMYRNARNBRNCRNDRNG\
             RNHRNKRNMRNNRNRRNSRNURNVRNWRNYRRARRBRRCRRDRRGRRHRRKRRMRRNRRRRRSRRURRVRRWRRYRSARSBRSCRS\
             DRSGRSHRSKRSMRSNRSRRSSRSURSVRSWRSYRUARUBRUCRUDRUGRUHRUKRUMRUNRURRUSRUURUVRUWRUYRVARVBR\
             VCRVDRVGRVHRVKRVMRVNRVRRVSRVURVVRVWRVYRWARWBRWCRWDRWGRWHRWKRWMRWNRWRRWSRWURWVRWWRWYRYA\
             RYBRYCRYDRYGRYHRYKRYMRYNRYRRYSRYURYVRYWRYYSAASABSACSADSAGSAHSAKSAMSANSARSASSAUSAVSAWSA\
             YSBASBBSBCSBDSBGSBHSBKSBMSBNSBRSBSSBUSBVSBWSBYSCASCBSCCSCDSCGSCHSCKSCMSCNSCRSCSSCUSCVS\
             CWSCYSDASDBSDCSDDSDGSDHSDKSDMSDNSDRSDSSDUSDVSDWSDYSGASGBSGCSGDSGGSGHSGKSGMSGNSGRSGSSGU\
             SGVSGWSGYSHASHBSHCSHDSHGSHHSHKSHMSHNSHRSHSSHUSHVSHWSHYSKASKBSKCSKDSKGSKHSKKSKMSKNSKRSK\
             SSKUSKVSKWSKYSMASMBSMCSMDSMGSMHSMKSMMSMNSMRSMSSMUSMVSMWSMYSNASNBSNCSNDSNGSNHSNKSNMSNNS\
             NRSNSSNUSNVSNWSNYSRASRBSRCSRDSRGSRHSRKSRMSRNSRRSRSSRUSRVSRWSRYSSASSBSSCSSDSSGSSHSSKSSM\
             SSNSSRSSSSSUSSVSSWSSYSUASUBSUCSUDSUGSUHSUKSUMSUNSURSUSSUUSUVSUWSUYSVASVBSVCSVDSVGSVHSV\
             KSVMSVNSVRSVSSVUSVVSVWSVYSWASWBSWCSWDSWGSWHSWKSWMSWNSWRSWSSWUSWVSWWSWYSYASYBSYCSYDSYGS\
             YHSYKSYMSYNSYRSYSSYUSYVSYWSYYUAAUABUACUADUAGUAHUAKUAMUANUARUASUAUUAVUAWUAYUBAUBBUBCUBD\
             UBGUBHUBKUBMUBNUBRUBSUBUUBVUBWUBYUCAUCBUCCUCDUCGUCHUCKUCMUCNUCRUCSUCUUCVUCWUCYUDAUDBUD\
             CUDDUDGUDHUDKUDMUDNUDRUDSUDUUDVUDWUDYUGAUGBUGCUGDUGGUGHUGKUGMUGNUGRUGSUGUUGVUGWUGYUHAU\
             HBUHCUHDUHGUHHUHKUHMUHNUHRUHSUHUUHVUHWUHYUKAUKBUKCUKDUKGUKHUKKUKMUKNUKRUKSUKUUKVUKWUKY\
             UMAUMBUMCUMDUMGUMHUMKUMMUMNUMRUMSUMUUMVUMWUMYUNAUNBUNCUNDUNGUNHUNKUNMUNNUNRUNSUNUUNVUN\
             WUNYURAURBURCURDURGURHURKURMURNURRURSURUURVURWURYUSAUSBUSCUSDUSGUSHUSKUSMUSNUSRUSSUSUU\
             SVUSWUSYUUAUUBUUCUUDUUGUUHUUKUUMUUNUURUUSUUUUUVUUWUUYUVAUVBUVCUVDUVGUVHUVKUVMUVNUVRUVS\
             UVUUVVUVWUVYUWAUWBUWCUWDUWGUWHUWKUWMUWNUWRUWSUWUUWVUWWUWYUYAUYBUYCUYDUYGUYHUYKUYMUYNUY\
             RUYSUYUUYVUYWUYYVAAVABVACVADVAGVAHVAKVAMVANVARVASVAUVAVVAWVAYVBAVBBVBCVBDVBGVBHVBKVBMV\
             BNVBRVBSVBUVBVVBWVBYVCAVCBVCCVCDVCGVCHVCKVCMVCNVCRVCSVCUVCVVCWVCYVDAVDBVDCVDDVDGVDHVDK\
             VDMVDNVDRVDSVDUVDVVDWVDYVGAVGBVGCVGDVGGVGHVGKVGMVGNVGRVGSVGUVGVVGWVGYVHAVHBVHCVHDVHGVH\
             HVHKVHMVHNVHRVHSVHUVHVVHWVHYVKAVKBVKCVKDVKGVKHVKKVKMVKNVKRVKSVKUVKVVKWVKYVMAVMBVMCVMDV\
             MGVMHVMKVMMVMNVMRVMSVMUVMVVMWVMYVNAVNBVNCVNDVNGVNHVNKVNMVNNVNRVNSVNUVNVVNWVNYVRAVRBVRC\
             VRDVRGVRHVRKVRMVRNVRRVRSVRUVRVVRWVRYVSAVSBVSCVSDVSGVSHVSKVSMVSNVSRVSSVSUVSVVSWVSYVUAVU\
             BVUCVUDVUGVUHVUKVUMVUNVURVUSVUUVUVVUWVUYVVAVVBVVCVVDVVGVVHVVKVVMVVNVVRVVSVVUVVVVVWVVYV\
             WAVWBVWCVWDVWGVWHVWKVWMVWNVWRVWSVWUVWVVWWVWYVYAVYBVYCVYDVYGVYHVYKVYMVYNVYRVYSVYUVYVVYW\
             VYYWAAWABWACWADWAGWAHWAKWAMWANWARWASWAUWAVWAWWAYWBAWBBWBCWBDWBGWBHWBKWBMWBNWBRWBSWBUWB\
             VWBWWBYWCAWCBWCCWCDWCGWCHWCKWCMWCNWCRWCSWCUWCVWCWWCYWDAWDBWDCWDDWDGWDHWDKWDMWDNWDRWDSW\
             DUWDVWDWWDYWGAWGBWGCWGDWGGWGHWGKWGMWGNWGRWGSWGUWGVWGWWGYWHAWHBWHCWHDWHGWHHWHKWHMWHNWHR\
             WHSWHUWHVWHWWHYWKAWKBWKCWKDWKGWKHWKKWKMWKNWKRWKSWKUWKVWKWWKYWMAWMBWMCWMDWMGWMHWMKWMMWM\
             NWMRWMSWMUWMVWMWWMYWNAWNBWNCWNDWNGWNHWNKWNMWNNWNRWNSWNUWNVWNWWNYWRAWRBWRCWRDWRGWRHWRKW\
             RMWRNWRRWRSWRUWRVWRWWRYWSAWSBWSCWSDWSGWSHWSKWSMWSNWSRWSSWSUWSVWSWWSYWUAWUBWUCWUDWUGWUH\
             WUKWUMWUNWURWUSWUUWUVWUWWUYWVAWVBWVCWVDWVGWVHWVKWVMWVNWVRWVSWVUWVVWVWWVYWWAWWBWWCWWDWW\
             GWWHWWKWWMWWNWWRWWSWWUWWVWWWWWYWYAWYBWYCWYDWYGWYHWYKWYMWYNWYRWYSWYUWYVWYWWYYYAAYABYACY\
             ADYAGYAHYAKYAMYANYARYASYAUYAVYAWYAYYBAYBBYBCYBDYBGYBHYBKYBMYBNYBRYBSYBUYBVYBWYBYYCAYCB\
             YCCYCDYCGYCHYCKYCMYCNYCRYCSYCUYCVYCWYCYYDAYDBYDCYDDYDGYDHYDKYDMYDNYDRYDSYDUYDVYDWYDYYG\
             AYGBYGCYGDYGGYGHYGKYGMYGNYGRYGSYGUYGVYGWYGYYHAYHBYHCYHDYHGYHHYHKYHMYHNYHRYHSYHUYHVYHWY\
             HYYKAYKBYKCYKDYKGYKHYKKYKMYKNYKRYKSYKUYKVYKWYKYYMAYMBYMCYMDYMGYMHYMKYMMYMNYMRYMSYMUYMV\
             YMWYMYYNAYNBYNCYNDYNGYNHYNKYNMYNNYNRYNSYNUYNVYNWYNYYRAYRBYRCYRDYRGYRHYRKYRMYRNYRRYRSYR\
             UYRVYRWYRYYSAYSBYSCYSDYSGYSHYSKYSMYSNYSRYSSYSUYSVYSWYSYYUAYUBYUCYUDYUGYUHYUKYUMYUNYURY\
             USYUUYUVYUWYUYYVAYVBYVCYVDYVGYVHYVKYVMYVNYVRYVSYVUYVVYVWYVYYWAYWBYWCYWDYWGYWHYWKYWMYWN\
             YWRYWSYWUYWVYWWYWYYYAYYBYYCYYDYYGYYHYYKYYMYYNYYRYYSYYUYYVYYWYYY")?;
        let protein = rna.convert(Kind::Protein)?;
        assert_eq!(
            protein,
            Seq::protein_iupac(
                "KXNXKXXXXKXNXXNXXXXXXXXXXXXXXXTTTTTTTTTTTTTTTXXXXXXXXXXXXXXXRXSXRXXXXRXSXXSXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XIXIXMIXIXXXIXIIXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXQXHXQXXXXQXHXXHXXXXXXXXXXXXXXXPPPPPPPPPPPP\
                 PPPXXXXXXXXXXXXXXXRRRRRRRRRRRRRRRXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXLLLLLLLLLLLLLLLXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXEX\
                 DXEXXXXEXDXXDXXXXXXXXXXXXXXXAAAAAAAAAAAAAAAXXXXXXXXXXXXXXXGGGGGGGGGGGGGGGXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXV\
                 VVVVVVVVVVVVVVXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXRXXXR\
                 XXXXRXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXBXXXXXXXXBXXBXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXZXXXZXXXXZXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXX*XYX*XXXX*XYXXYXXXXXXXXXXXXXXXSSSSSSSSSSSSSSSXXXXXXXXXXXXXXX*XCXWXX\
                 XXXXCXXCXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX*XXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXLXFXLXXXXLXFXXFXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXLXXXLXXXXLXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX\
                 XXXXXXXXXXXXX")?,
        );
        Ok(())
    }

    #[test]
    fn dna_to_protein() -> Result<(), Error> {
        let dna = Seq::dna("ATGGCCATGGCGCCCAGAACTGAGATCAATAGTACCCGTATTAACGGGTGA")?;
        let protein = dna.convert(Kind::Protein)?;
        assert_eq!(protein, Seq::protein("MAMAPRTEINSTRING*")?);
        Ok(())
    }

    #[test]
    fn dna_lowercase_to_protein() -> Result<(), Error> {
        let dna = Seq::dna("atggccatggcgcccagaactgagatcaatagtacccgtattaacgggtga")?;
        let protein = dna.convert(Kind::Protein)?;
        assert_eq!(protein, Seq::protein("MAMAPRTEINSTRING*")?);
        Ok(())
    }

    #[test]
    fn rna_ambiguous_to_protein() -> Result<(), Error> {
        let rna = Seq::rna_n("AUGGCNAUGGCNCCNAGAACUGAGAUCAAUAGUACNCGUAUUAACGGNUGA")?;
        let protein = rna.convert(Kind::Protein)?;
        assert_eq!(protein, Seq::protein("MAMAPRTEINSTRING*")?);
        let rna = Seq::rna_iupac("AUGGCVAUGGCNCCNAGAACUGARAUCAAYAGUACNCGUAUHAAYGGNUAR")?;
        let protein = rna.convert(Kind::Protein)?;
        assert_eq!(protein, Seq::protein("MAMAPRTEINSTRING*")?);
        Ok(())
    }

    #[test]
    fn rna_ambiguous_to_ambiguous_protein() -> Result<(), Error> {
        let rna = Seq::rna_n("AUGGCCAUGGCGNCCAGAACUGAGAUCAAUNGUACCCGUAUNAACGGGNGA")?;
        let protein = rna.convert(Kind::Protein)?;
        assert_eq!(protein, Seq::protein_iupac("MAMAXRTEINXTRXNGX")?);
        let rna = Seq::rna_iupac("AUGRAYAUBGCGNCCAGAACUGAGAUCAAUWGUACCCGUAUNASCSARNGA")?;
        let protein = rna.convert(Kind::Protein)?;
        assert_eq!(protein, Seq::protein_iupac("MBXAXRTEINXTRXXZX")?);
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

    // ===== Reverse Complement Tool Tests =========================================================

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

    // ===== Error Formatting Tests ================================================================

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
        assert_eq!(
            Error::InvalidSeq(vec![
                (Kind::Dna, Alphabet::Base),
                (Kind::Rna, Alphabet::Base),
                (Kind::Dna, Alphabet::N),
                (Kind::Rna, Alphabet::N),
                (Kind::Protein, Alphabet::Base),
                (Kind::Dna, Alphabet::Iupac),
                (Kind::Rna, Alphabet::Iupac),
                (Kind::Protein, Alphabet::Iupac)
            ])
            .to_string(),
            "The provided sequence was not valid DNA (Base), RNA (Base), DNA (N), RNA (N), Protein (Base), DNA (IUPAC), RNA (IUPAC), or Protein (IUPAC)"
        );
        assert_eq!(
            Error::InvalidSeq(vec![
                (Kind::Dna, Alphabet::Base),
                (Kind::Dna, Alphabet::N),
                (Kind::Dna, Alphabet::Iupac),
            ])
            .to_string(),
            "The provided sequence was not valid DNA (Base), DNA (N), or DNA (IUPAC)"
        );
        assert_eq!(
            Error::InvalidSeq(vec![
                (Kind::Rna, Alphabet::Base),
                (Kind::Rna, Alphabet::N),
                (Kind::Rna, Alphabet::Iupac),
            ])
            .to_string(),
            "The provided sequence was not valid RNA (Base), RNA (N), or RNA (IUPAC)"
        );
        assert_eq!(
            Error::InvalidSeq(vec![
                (Kind::Protein, Alphabet::Base),
                (Kind::Protein, Alphabet::Iupac)
            ])
            .to_string(),
            "The provided sequence was not valid Protein (Base), or Protein (IUPAC)"
        );
        assert_eq!(
            &Error::RevComp(Kind::Protein).to_string(),
            "Cannot reverse complement Protein"
        );
    }
}
