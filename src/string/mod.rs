pub mod hash;
pub mod iter;

use hash::CharHasher;
use iter::StringTrieIter;

use crate::{
    node::TrieNode,
    trie::Trie,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

/// A string trie wrapper.
#[derive(Default)]
pub struct StringTrie(Trie<char, CharHasher>);

impl StringTrie {
    /// Returns a new empty string trie.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns `true` if no values are stored within the trie.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of values within the trie.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Clears the trie.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Inserts a string into the trie.
    pub fn insert(&mut self, value: &str) -> bool {
        self.0.insert(value.chars())
    }

    /// Inserts a string into the trie.
    pub fn insert_iter(&mut self, iter: impl Iterator<Item = char>) -> bool {
        self.0.insert(iter)
    }

    /// Removes a string from the trie.
    pub fn remove(&mut self, value: &str) -> bool {
        self.0.remove(value.chars())
    }

    /// Removes a string from the trie.
    pub fn remove_iter(&mut self, iter: impl Iterator<Item = char>) -> bool {
        self.0.remove(iter)
    }

    /// Returns `true` if the trie contains the string, otherwise returns
    /// `false`.
    pub fn contains(&self, value: &str) -> bool {
        self.0.contains(value.chars())
    }

    /// Returns `true` if the trie contains the string, otherwise returns
    /// `false`.
    pub fn contains_iter(&self, iter: impl Iterator<Item = char>) -> bool {
        self.0.contains(iter)
    }

    /// Returns an immutable reference to the root [`TrieNode`].
    pub fn root(&self) -> &TrieNode<char, CharHasher> {
        self.0.root()
    }

    /// Returns an iterator every [`String`] in the trie.
    pub fn iter(&self) -> impl Iterator<Item = String> + use<'_> {
        StringTrieIter::new(self.0.root())
    }
}

#[cfg(feature = "serde")]
impl Serialize for StringTrie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for StringTrie {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let trie = Trie::<char, CharHasher>::deserialize(deserializer)?;
        Ok(StringTrie(trie))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let trie = StringTrie::new();
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn test_insertion() {
        let mut trie = StringTrie::new();
        assert!(trie.insert("test"));

        assert!(trie.contains("test"));

        assert!(!trie.contains("tests"));
        assert!(!trie.contains("tes"));
        assert!(!trie.contains("te"));
        assert!(!trie.contains("t"));

        assert!(!trie.contains("other"));

        // Try add the word again, this should return false since the word
        // already exits:
        assert!(!trie.insert("test"));
    }

    #[test]
    fn test_len() {
        let mut trie = StringTrie::new();
        assert_eq!(trie.len(), 0);
        
        // Test insertions:
        assert!(trie.insert("a"));
        assert_eq!(trie.len(), 1);

        assert!(!trie.insert("a"));
        assert_eq!(trie.len(), 1);

        assert!(trie.insert("b"));
        assert_eq!(trie.len(), 2);

        assert!(trie.insert("c"));
        assert_eq!(trie.len(), 3);

        assert!(!trie.insert("a"));
        assert_eq!(trie.len(), 3);

        // Test deletions:
        assert!(trie.remove("a"));
        assert_eq!(trie.len(), 2);

        // Test deletions:
        assert!(trie.remove("c"));
        assert_eq!(trie.len(), 1);

        // Test deletions:
        assert!(trie.remove("b"));
        assert_eq!(trie.len(), 0);

        // Test deletions:
        assert!(!trie.remove("a"));
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn test_deletion() {
        let mut trie = StringTrie::new();
        assert!(trie.insert("test"));
        assert!(trie.insert("testing"));
        assert!(trie.insert("tester"));
        assert!(trie.insert("tesla"));
        assert!(trie.insert("tech"));

        // Remove "test":
        assert!(trie.remove("test"));
        assert!(!trie.remove("test"));
        assert!(trie.contains("testing"));
        assert!(trie.contains("tester"));
        assert!(trie.contains("tesla"));
        assert!(trie.contains("tech"));

        // Remove "tester":
        assert!(trie.remove("tester"));
        assert!(!trie.contains("tester"));
        assert!(!trie.contains("test"));
        assert!(trie.contains("testing"));
        assert!(trie.contains("tesla"));
        assert!(trie.contains("tech"));

        // Remove "tech":
        assert!(trie.remove("tech"));
        assert!(!trie.contains("tech"));
        assert!(trie.contains("testing"));
        assert!(trie.contains("tesla"));

        // Remove testing:
        assert!(trie.remove("testing"));
        assert!(!trie.contains("testing"));
        assert!(trie.contains("tesla"));

        // Remove tesla:
        assert!(trie.remove("tesla"));
        assert!(!trie.contains("tesla"));
        assert!(trie.is_empty());
    }

    #[test]
    fn test_insertion_and_deletion() {
        let mut trie = StringTrie::new();
        assert!(trie.insert("test"));
        assert!(trie.insert("testing"));
        assert!(trie.insert("tester"));

        assert!(trie.contains("test"));

        assert!(trie.remove("test"));
        assert!(!trie.contains("test"));

        assert!(!trie.remove("test"));
        assert!(!trie.contains("test"));
        assert!(trie.contains("testing"));
        assert!(trie.contains("tester"));

        assert!(trie.insert("test"));
        assert!(trie.contains("test"));
        assert!(trie.contains("testing"));
        assert!(trie.contains("tester"));

        assert!(trie.remove("testing"));
        assert!(trie.contains("test"));
        assert!(!trie.contains("testing"));
        assert!(trie.contains("tester"));
    }

    #[test]
    fn test_iter() {
        let mut values = std::collections::HashSet::new();
        values.insert("test");
        values.insert("testing");
        values.insert("tester");
        values.insert("other");
        values.insert("hello world");

        let mut trie = StringTrie::new();
        for value in values.iter() {
            assert!(trie.insert(value));
        }

        for value in trie.iter() {
            assert!(values.remove(value.as_str()));
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        use serde_json;

        // Create a StringTrie and insert some values
        let mut trie = StringTrie::new();
        assert!(trie.insert("hello"));
        assert!(trie.insert("world"));
        assert!(trie.insert("trie"));
        assert!(trie.insert("serialize"));
        assert!(trie.insert("deserialize"));

        // Serialize the trie to a JSON string
        let serialized = serde_json::to_string(&trie).unwrap();
        println!("Serialized StringTrie: {}", serialized);

        // Deserialize the JSON string back into a StringTrie
        let deserialized: StringTrie = serde_json::from_str(&serialized).unwrap();

        // Verify that the deserialized trie contains the same data
        assert!(deserialized.contains("hello"));
        assert!(deserialized.contains("world"));
        assert!(deserialized.contains("trie"));
        assert!(deserialized.contains("serialize"));
        assert!(deserialized.contains("deserialize"));

        // Ensure deserialized trie has the correct length
        assert_eq!(deserialized.len(), 5);

        // Ensure it does not contain non-existent words
        assert!(!deserialized.contains("missing"));
        assert!(!deserialized.contains("data"));
    }
}