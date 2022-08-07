use std::{
    cell::Cell,
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    iter::FusedIterator,
    ops::{Index, IndexMut},
};

use once_cell::unsync::OnceCell;

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

// TODO: Would iterating over &u8 help or hurt here? Surely a reference is actually a bigger type
// than u8 is on its own. I don't see why copying u8's would be slower than constructing
// references, but still should be tested!
// OPTIMI
// OPTIMISATION: Restricting this trait further (adding others like `ExactSizeIterator`) could be
// helpful for optimisation by the compiler, but the use of `take_while()` has forced the bounds to
// be opened to just `Iterator` and `FusedIterator`
trait ByteIter: Debug + FusedIterator<Item = u8> + Iterator<Item = u8> {}

// TODO: This could be more cleanly implemented using the lazy module of the standard library once
// that has been stabilised. Keep an eye on https://github.com/rust-lang/rust/issues/74465
// TODO: The naming and fields here generally could use a second look!
// TODO: I'll probably want to add some derives at some point, or implement them manually!
// TODO: Could just call this `Lazy` and make it work for non-Vec types too (just chaining init
// functions)
// TODO: This should implement Deref, like the Lazy type does!
pub struct LazyVec<T> {
    vec: OnceCell<Vec<T>>,
    partial: dyn ByteIter,
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
