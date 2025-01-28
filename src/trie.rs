use std::hash::{
    BuildHasher,
    Hash,
    RandomState,
};

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
    ser::SerializeStruct,
};

use crate::node::TrieNode;

/// Stores a list of words efficiently in memory.
/// 
/// ## How Does it Work?
/// This is a memory optimised word storage type with fast lookups. This is
/// achieved by storing each individual letter of a word and any children. Each
/// node stores if it is the end of a word or not.
/// 
/// For example, when inserting the word "test", it will create the following
/// branch of nodes:
/// t -> e -> s -> t
/// 
/// If we then add the word "teach", it will create a sub-branch:
/// t -> e -> s -> t
///           a -> c -> h
/// 
/// Finally, if we add the word "testing", it will extend the original branch to
/// include the new word:
/// t -> e -> s -> t -> i -> n -> g
///           a -> c -> h
/// 
/// Each node stores a boolean to indicate if it is the end of a word. This
/// allows for fast word lookups. This will be indicated with a capital letter
/// for demonstration purposes using the previously generated tree:
/// t -> e -> s -> T -> i -> n -> G
///           a -> c -> H
/// 
/// ## Important Notes
/// This type is case sensitive. Any words inserted into the type must be
/// sanitised before being entered.
/// 
/// ## What is this Useful For?
/// If you need to quickly check a string against a very large number of strings
/// very quickly.
#[derive(Default)]
pub struct Trie<T, H = RandomState> {
    /// Root node that tracks every value within the trie.
    root: TrieNode<T, H>,
    /// Tracks the number of values in the trie.
    count: usize,
}

impl<T, H> Trie<T, H>
where
    T: Hash + Eq,
    H: BuildHasher + Default,
{
    /// Creates a new empty [`Trie`].
    pub fn new() -> Self {
        Self {
            root: TrieNode::empty(),
            count: 0,
        }
    }

    /// Returns `true` if the trie is empty, otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    /// Returns the number of values within the trie.
    pub fn len(&self) -> usize {
        self.count
    }

    /// Clears the trie.
    pub fn clear(&mut self) {
        self.root.clear();
    }

    /// Inserts a value into the trie.
    /// 
    /// This function returns `true` if the value that was added is a newly
    /// added value, otherwise returns `false`. This function will also return
    /// `false` if the value provided was empty.
    pub fn insert(&mut self, iter: impl IntoIterator<Item = T>) -> bool {
        // Walk the iterator until we reach the end:
        let mut current = &mut self.root;
        for char in iter.into_iter() {
            current = current.get_or_create(char);
        }

        // Determine if it is a new value and update the internal value counter:
        let is_new = !current.end_of_value;
        if is_new {
            self.count += 1;
        }

        // Finalise:
        current.end_of_value = true;
        is_new
    }

    /// Removes a value from the trie.
    /// 
    /// This function returns `true` if the value was successfully removed,
    /// otherwise if the value doesn't exist, this returns `false`. This
    /// function will also return `false` if the value provided was empty.
    pub fn remove(&mut self, iter: impl IntoIterator<Item = T>) -> bool {
        let removed = self.root.remove_branch(iter.into_iter());
        if removed {
            self.count -= 1;
        }
        removed
    }

    /// Checks if the [`Trie`] contains a value.
    /// 
    /// If the value exists, this function returns `true`, otherwise it returns
    /// `false`. This function will also return false if the value provided is
    /// empty.
    pub fn contains(&self, iter: impl IntoIterator<Item = T>) -> bool {
        let mut current = &self.root;
        for char in iter.into_iter() {
            match current.get(&char) {
                Some(next_node) => {
                    current = next_node
                },
                None => {
                    return false;
                },
            }
        }
        current.end_of_value
    }

    /// Returns an immutable reference to the root [`TrieNode`].
    /// 
    /// This is the node that contains every value.
    pub fn root(&self) -> &TrieNode<T, H> {
        &self.root
    }
}

impl<T, H> Trie<T, H>
where
    T: Eq + Copy,
{
    /// Returns an iterator over the [`Trie`].
    pub fn iter(&self) -> impl Iterator<Item = Vec<T>> + use<'_, T, H> {
        self.root.iter()
    }
}

#[cfg(feature = "serde")]
impl<T, H> Serialize for Trie<T, H>
where
    T: Hash + Eq + Serialize,
    H: BuildHasher + Default,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as a struct with two fields: `root` and `count`
        let mut state = serializer.serialize_struct("Trie", 2)?;
        state.serialize_field("root", &self.root)?;
        state.serialize_field("count", &self.count)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T, H> Deserialize<'de> for Trie<T, H>
where
    T: Hash + Eq + Deserialize<'de>,
    H: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Visitor for deserializing a `Trie`.
        struct TrieVisitor<T, H> {
            marker: std::marker::PhantomData<(T, H)>,
        }

        impl<'de, T, H> serde::de::Visitor<'de> for TrieVisitor<T, H>
        where
            T: Hash + Eq + Deserialize<'de>,
            H: BuildHasher + Default,
        {
            type Value = Trie<T, H>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Trie with root and count fields")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut root = None;
                let mut count = None;

                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "root" => {
                            if root.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root = Some(map.next_value()?);
                        }
                        "count" => {
                            if count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            count = Some(map.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(key, &["root", "count"]));
                        }
                    }
                }

                let root = root.ok_or_else(|| serde::de::Error::missing_field("root"))?;
                let count = count.ok_or_else(|| serde::de::Error::missing_field("count"))?;

                Ok(Trie { root, count })
            }
        }

        deserializer.deserialize_struct(
            "Trie",
            &["root", "count"],
            TrieVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type CharTrie = Trie<char>;

    #[test]
    fn test_empty() {
        let trie = CharTrie::new();
        assert!(trie.is_empty());
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn test_insertion() {
        let mut trie = CharTrie::new();
        assert!(trie.insert("test".chars()));

        assert!(trie.contains("test".chars()));

        assert!(!trie.contains("tests".chars()));
        assert!(!trie.contains("tes".chars()));
        assert!(!trie.contains("te".chars()));
        assert!(!trie.contains("t".chars()));

        assert!(!trie.contains("other".chars()));

        // Try add the word again, this should return false since the word
        // already exits:
        assert!(!trie.insert("test".chars()));
    }

    #[test]
    fn test_len() {
        let mut trie = CharTrie::new();
        assert_eq!(trie.len(), 0);
        
        // Test insertions:
        assert!(trie.insert("a".chars()));
        assert_eq!(trie.len(), 1);

        assert!(!trie.insert("a".chars()));
        assert_eq!(trie.len(), 1);

        assert!(trie.insert("b".chars()));
        assert_eq!(trie.len(), 2);

        assert!(trie.insert("c".chars()));
        assert_eq!(trie.len(), 3);

        assert!(!trie.insert("a".chars()));
        assert_eq!(trie.len(), 3);

        // Test deletions:
        assert!(trie.remove("a".chars()));
        assert_eq!(trie.len(), 2);

        // Test deletions:
        assert!(trie.remove("c".chars()));
        assert_eq!(trie.len(), 1);

        // Test deletions:
        assert!(trie.remove("b".chars()));
        assert_eq!(trie.len(), 0);

        // Test deletions:
        assert!(!trie.remove("a".chars()));
        assert_eq!(trie.len(), 0);
    }

    #[test]
    fn test_deletion() {
        let mut trie = CharTrie::new();
        assert!(trie.insert("test".chars()));
        assert!(trie.insert("testing".chars()));
        assert!(trie.insert("tester".chars()));
        assert!(trie.insert("tesla".chars()));
        assert!(trie.insert("tech".chars()));

        // Remove "test":
        assert!(trie.remove("test".chars()));
        assert!(!trie.remove("test".chars()));
        assert!(trie.contains("testing".chars()));
        assert!(trie.contains("tester".chars()));
        assert!(trie.contains("tesla".chars()));
        assert!(trie.contains("tech".chars()));

        // Remove "tester":
        assert!(trie.remove("tester".chars()));
        assert!(!trie.contains("tester".chars()));
        assert!(!trie.contains("test".chars()));
        assert!(trie.contains("testing".chars()));
        assert!(trie.contains("tesla".chars()));
        assert!(trie.contains("tech".chars()));

        // Remove "tech":
        assert!(trie.remove("tech".chars()));
        assert!(!trie.contains("tech".chars()));
        assert!(trie.contains("testing".chars()));
        assert!(trie.contains("tesla".chars()));

        // Remove testing:
        assert!(trie.remove("testing".chars()));
        assert!(!trie.contains("testing".chars()));
        assert!(trie.contains("tesla".chars()));

        // Remove tesla:
        assert!(trie.remove("tesla".chars()));
        assert!(!trie.contains("tesla".chars()));
        assert!(trie.is_empty());
    }

    #[test]
    fn test_insertion_and_deletion() {
        let mut trie = CharTrie::new();
        assert!(trie.insert("test".chars()));
        assert!(trie.insert("testing".chars()));
        assert!(trie.insert("tester".chars()));

        assert!(trie.contains("test".chars()));

        assert!(trie.remove("test".chars()));
        assert!(!trie.contains("test".chars()));

        assert!(!trie.remove("test".chars()));
        assert!(!trie.contains("test".chars()));
        assert!(trie.contains("testing".chars()));
        assert!(trie.contains("tester".chars()));

        assert!(trie.insert("test".chars()));
        assert!(trie.contains("test".chars()));
        assert!(trie.contains("testing".chars()));
        assert!(trie.contains("tester".chars()));

        assert!(trie.remove("testing".chars()));
        assert!(trie.contains("test".chars()));
        assert!(!trie.contains("testing".chars()));
        assert!(trie.contains("tester".chars()));
    }

    #[test]
    fn test_iter() {
        let mut values = std::collections::HashSet::new();
        values.insert("test");
        values.insert("testing");
        values.insert("tester");
        values.insert("other");
        values.insert("hello world");

        let mut trie = CharTrie::new();
        for value in values.iter() {
            assert!(trie.insert(value.chars()));
        }

        for value in trie.iter() {
            let value = String::from_iter(value.iter());
            assert!(values.remove(value.as_str()));
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        use serde_json;

        let mut trie = CharTrie::new();
        assert!(trie.insert("hello".chars()));
        assert!(trie.insert("world".chars()));
        assert!(trie.insert("trie".chars()));
        assert!(trie.insert("serialize".chars()));
        assert!(trie.insert("deserialize".chars()));

        // Serialize the trie to a JSON string
        let serialized = serde_json::to_string(&trie).unwrap();
        println!("Serialized Trie::<char>: {}", serialized);

        // Deserialize the JSON string back into a StringTrie
        let deserialized: CharTrie = serde_json::from_str(&serialized).unwrap();

        // Verify that the deserialized trie contains the same data
        assert!(deserialized.contains("hello".chars()));
        assert!(deserialized.contains("world".chars()));
        assert!(deserialized.contains("trie".chars()));
        assert!(deserialized.contains("serialize".chars()));
        assert!(deserialized.contains("deserialize".chars()));

        // Ensure deserialized trie has the correct length
        assert_eq!(deserialized.len(), 5);

        // Ensure it does not contain non-existent words
        assert!(!deserialized.contains("missing".chars()));
        assert!(!deserialized.contains("data".chars()));
    }
}