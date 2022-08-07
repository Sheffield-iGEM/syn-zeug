use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Case {
    Upper,
    Lower,
}

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
    use crate::{
        data::ALPHABET_MAP,
        seq::{Alphabet, Kind},
    };

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
