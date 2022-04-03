use bio::alphabets::{dna, protein, rna, Alphabet};
use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};
// Keep an eye on this: https://github.com/rust-lang/rust/issues/74465
use once_cell::sync::Lazy;

use crate::seq::SeqKind;

pub static ALPHABETS: Lazy<HashMap<SeqKind, Alphabet>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(SeqKind::Dna, dna::iupac_alphabet());
    m.insert(SeqKind::Rna, rna::iupac_alphabet());
    m.insert(SeqKind::Protein, protein::iupac_alphabet());
    m
});

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
