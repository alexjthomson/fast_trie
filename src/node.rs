use std::{borrow::Borrow, collections::HashMap, hash::{BuildHasher, Hash}};

/// A node within a trie.
#[derive(Default)]
pub struct TrieNode<T, H> {
    /// Child nodes for each character.
    children: HashMap<T, Self, H>,
    /// Tracks if the current character is the end of a word.
    pub(super) end_of_value: bool,
}

impl<T, H> TrieNode<T, H>
where
    T: Hash + Eq,
    H: BuildHasher + Default,
{
    /// Creates a new empty [`TrieNode`].
    pub fn empty() -> Self {
        Self {
            children: HashMap::with_hasher(Default::default()),
            end_of_value: false,
        }
    }

    /// Returns `true` if the [`TrieNode`] contains no child nodes.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns the number of child [`TrieNode`]s in this node.
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// Clears the [`TrieNode`].
    pub(super) fn clear(&mut self) {
        self.children.clear();
        self.end_of_value = false;
    }

    /// Returns an immutable reference to the child [`TrieNode`] for the given
    /// `value`.
    /// 
    /// If no child node is found, this function returns [`None`].
    pub fn get(&self, value: &T) -> Option<&Self> {
        self
            .children
            .get(value)
    }

    /// Gets or creates a child node for a given character.
    /// 
    /// This function assumes that the value passed into it is a lowercase
    /// alphabetic English character.
    #[inline]
    #[must_use]
    pub(super) fn get_or_create(&mut self, value: T) -> &mut Self {
        self
            .children
            .entry(value)
            .or_insert_with(Self::empty)
    }

    /// Removes a branch of children from this [`TrieNode`].
    /// 
    /// Returns `true` if the branch was successfully removed; otherwise, if no
    /// child branch existed for the given `value`, this function returns
    /// `false`.
    pub(super) fn remove_branch<E>(&mut self, iter: impl Iterator<Item = E>) -> bool
    where
        E: Borrow<T>,
    {
        self.remove_internal(iter, &|node, element| {
            // Check if the node can be removed:
            if node.children.get(element.borrow()).unwrap().can_remove() {
                // Remove the node:
                let result = node.children.remove(element.borrow());
                debug_assert!(matches!(result, Some(..)))
            }
        })
    }

    /// Returns `true` if this node can be safely removed from its parent.
    fn can_remove(&self) -> bool {
        !self.end_of_value && self.is_empty()
    }

    fn remove_internal<E>(
        &mut self,
        mut iter: impl Iterator<Item = E>,
        remove_fn: &impl Fn(&mut Self, E),
    ) -> bool
    where
        E: Borrow<T>,
    {
        match iter.next() {
            Some(element) => {
                // There was a next element in the iterator, we should check if
                // we can remove it:
                match self.children.get_mut(element.borrow()) {
                    Some(next_node) => {
                        // We found the next node this element points to, we
                        // should try remove the remaining branch from it and
                        // return the success state:
                        let is_removed = next_node.remove_internal(iter, remove_fn);
                        remove_fn(self, element);
                        is_removed
                    },
                    None => {
                        // There was no next node, this value therefore doesn't
                        // exist in the node.
                        false
                    },
                }
            },
            None => {
                // We have reached the end of the iterator, therefore we must be
                // at the end of the value. If this current node is the end of a
                // value, we have successfully found the end and can return true
                // to indicate the value was removed. We need to ensure this
                // node is no longer marked as the end of a value.
                let is_end_of_value = self.end_of_value;
                if is_end_of_value {
                    self.end_of_value = false;
                }
                is_end_of_value
            }
        }
    }

    /// Returns `true` if this node forms the end of a word; otherwise returns
    /// `false`.
    pub fn is_end_of_word(&self) -> bool {
        self.end_of_value
    }
}