use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::Display,
    hash::Hash,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Dna {
    A,
    C,
    G,
    T,
}

impl TryFrom<char> for Dna {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'A' => Ok(Self::A),
            'c' | 'C' => Ok(Self::C),
            'g' | 'G' => Ok(Self::G),
            't' | 'T' => Ok(Self::T),
            c => Err(format!(
                "The character '{}' is not a valid DNA nucleobase",
                c
            )),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Rna {
    A,
    C,
    G,
    U,
}

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        match dna {
            Dna::A => Rna::A,
            Dna::C => Rna::C,
            Dna::G => Rna::G,
            Dna::T => Rna::U,
        }
    }
}

impl<T: Display> Display for Vec<T> {}

// FIXME: This is a nasty API? I might need to abstract over "sequences". Ideally this would
// collect into the same type it was originally passed?
pub fn map_into<T, U: From<T>>(seq: impl IntoIterator<Item = T>) -> Vec<U> {
    seq.into_iter().map(|x| x.into()).collect()
}

// FIXME: This feels like it belongs in a trait! Could also be more generic!
pub fn read_sequence<T: TryFrom<char>>(seq: impl AsRef<str>) -> Result<Vec<T>, T::Error> {
    seq.as_ref().chars().map(|c| c.try_into()).collect()
}

// FIXME: This needs a better name
pub fn count_components<T: Hash + Eq>(seq: &[T]) -> HashMap<&T, usize> {
    let mut map = HashMap::new();
    for i in seq {
        *map.entry(i).or_default() += 1;
    }
    map
}

#[cfg(test)]
// FIXME: These could probably be a bit less repetitive...
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn read_valid_dna_bases() {
        assert_eq!('a'.try_into(), Ok(Dna::A));
        assert_eq!('A'.try_into(), Ok(Dna::A));
        assert_eq!('c'.try_into(), Ok(Dna::C));
        assert_eq!('C'.try_into(), Ok(Dna::C));
        assert_eq!('g'.try_into(), Ok(Dna::G));
        assert_eq!('G'.try_into(), Ok(Dna::G));
        assert_eq!('t'.try_into(), Ok(Dna::T));
        assert_eq!('T'.try_into(), Ok(Dna::T));
    }

    #[test]
    fn read_invalid_dna_bases() {
        let x: Result<Dna, String> = 'X'.try_into();
        let n: Result<Dna, String> = 'n'.try_into();
        assert_eq!(
            x,
            Err("The character 'X' is not a valid DNA nucleobase".into())
        );
        assert_eq!(
            n,
            Err("The character 'n' is not a valid DNA nucleobase".into())
        );
    }

    #[test]
    fn read_valid_dna_sequence() {
        let dna = "AGCTTTTCATTCTGACTGCA";
        assert_eq!(
            read_sequence(dna),
            Ok(vec![
                Dna::A,
                Dna::G,
                Dna::C,
                Dna::T,
                Dna::T,
                Dna::T,
                Dna::T,
                Dna::C,
                Dna::A,
                Dna::T,
                Dna::T,
                Dna::C,
                Dna::T,
                Dna::G,
                Dna::A,
                Dna::C,
                Dna::T,
                Dna::G,
                Dna::C,
                Dna::A
            ])
        );
    }

    #[test]
    fn read_invalid_dna_sequence() {
        let dna = "AGCTTTXCATTCTGACNGCA";
        let dna: Result<Vec<Dna>, String> = read_sequence(dna);
        assert_eq!(
            dna,
            Err("The character 'X' is not a valid DNA nucleobase".into())
        );
    }

    #[test]
    fn count_nucleotides() -> Result<(), String> {
        let dna = read_sequence(
            "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC",
        )?;
        let counts = count_components(&dna);
        assert_eq!(counts.get(&Dna::A), Some(&20));
        assert_eq!(counts.get(&Dna::C), Some(&12));
        assert_eq!(counts.get(&Dna::G), Some(&17));
        assert_eq!(counts.get(&Dna::T), Some(&21));
        Ok(())
    }

    #[test]
    fn dna_to_rna() {
        let rna: Rna = Dna::A.into();
        assert_eq!(rna, Rna::A);
        let rna: Rna = Dna::C.into();
        assert_eq!(rna, Rna::C);
        let rna: Rna = Dna::G.into();
        assert_eq!(rna, Rna::G);
        let rna: Rna = Dna::T.into();
        assert_eq!(rna, Rna::U);
    }

    #[test]
    fn dna_sequence_to_rna_sequence() -> Result<(), String> {
        let dna: Vec<Dna> = read_sequence("GATGGAACTTGACTACGTAAATT")?;
        let rna: Vec<Rna> = map_into(dna);
        assert_eq!(
            rna,
            vec![
                Rna::G,
                Rna::A,
                Rna::U,
                Rna::G,
                Rna::G,
                Rna::A,
                Rna::A,
                Rna::C,
                Rna::U,
                Rna::U,
                Rna::G,
                Rna::A,
                Rna::C,
                Rna::U,
                Rna::A,
                Rna::C,
                Rna::G,
                Rna::U,
                Rna::A,
                Rna::A,
                Rna::A,
                Rna::U,
                Rna::U
            ]
        );
        Ok(())
    }
}
