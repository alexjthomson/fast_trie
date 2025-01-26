use std::hash::{BuildHasher, Hash};

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
pub struct Trie<T, H> {
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

// TODO: Add unit tests