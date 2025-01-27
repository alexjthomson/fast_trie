use std::hash::{
    BuildHasher,
    Hasher,
};

/// A [`Hasher`] optimised for hashing [`char`]s using non-cryptographic hashes.
#[derive(Default)]
pub struct CharHasher(u64);

impl Hasher for CharHasher {
    fn write(&mut self, _bytes: &[u8]) {
        unimplemented!("This hasher only supports `u32` inputs for simplicity.")
    }

    fn write_u32(&mut self, i: u32) {
        self.0 = i as u64;
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

impl BuildHasher for CharHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}