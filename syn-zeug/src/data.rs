use bio::alphabets::{dna, protein, rna};
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};
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
        let mut m = HashMap::new();
        m.insert((Kind::Dna, Alphabet::Base), dna::alphabet());
        m.insert((Kind::Dna, Alphabet::N), dna::n_alphabet());
        m.insert((Kind::Dna, Alphabet::Iupac), dna::iupac_alphabet());

        m.insert((Kind::Rna, Alphabet::Base), rna::alphabet());
        m.insert((Kind::Rna, Alphabet::N), rna::n_alphabet());
        m.insert((Kind::Rna, Alphabet::Iupac), rna::iupac_alphabet());

        m.insert((Kind::Protein, Alphabet::Base), protein::alphabet());
        m.insert((Kind::Protein, Alphabet::Iupac), protein::iupac_alphabet());
        m
    });

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ByteMap<T>([T; 128]);

impl<T: Copy> ByteMap<T> {
    pub fn new(default: T) -> Self {
        Self([default; 128])
    }

    pub fn to_hashmap<U: From<u8> + Hash + Eq>(&self, f: impl Fn(&U, &T) -> bool) -> HashMap<U, T> {
        self.0
            .into_iter()
            .enumerate()
            .map(|(i, t)| ((i as u8).into(), t))
            .filter(|(u, t)| f(u, t))
            .collect()
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
    fn bytemap_to_hashmap_by_value() {
        let mut counts = ByteMap::default();
        counts[b'A'] = 20;
        counts[b'C'] = 12;
        counts[b'G'] = 17;
        counts[b'T'] = 21;
        let counts: HashMap<char, _> = counts.to_hashmap(|_, &x| x != 0);
        assert_eq!(counts.len(), 4);
        assert_eq!(counts[&'A'], 20);
        assert_eq!(counts[&'C'], 12);
        assert_eq!(counts[&'G'], 17);
        assert_eq!(counts[&'T'], 21);
    }

    #[test]
    fn bytemap_to_hashmap_by_key() {
        let mut counts = ByteMap::default();
        counts[b'A'] = 20;
        counts[b'G'] = 17;
        counts[b'T'] = 21;
        let counts: HashMap<char, _> = counts.to_hashmap(|&b, _| {
            ALPHABET_MAP[&(Kind::Dna, Alphabet::Base)]
                .symbols
                .contains(b as usize)
        });
        assert_eq!(counts.len(), 8); // Lowercase 'a', 'c', 'g', and 't' are also included
        assert_eq!(counts[&'A'], 20);
        assert_eq!(counts[&'C'], 0);
        assert_eq!(counts[&'G'], 17);
        assert_eq!(counts[&'T'], 21);
    }
}
