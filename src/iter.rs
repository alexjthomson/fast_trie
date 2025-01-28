use crate::node::TrieNode;

/// Iterates over a [`TrieNode`].
pub struct TrieIter<'a, T, H> {
    stack: Vec<(&'a TrieNode<T, H>, usize)>,
    buffer: Vec<&'a T>,
}

impl<'a, T, H> TrieIter<'a, T, H> {
    /// Creates a new [`TrieIter`] over a root [`TrieNode`].
    pub fn new(root: &'a TrieNode<T, H>) -> Self {
        Self {
            stack: vec![(root, 0)],
            buffer: Vec::new(),
        }
    }
}

impl<'a, T, H> Iterator for TrieIter<'a, T, H>
where
    T: Eq,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, depth)) = self.stack.pop() {
            // If we're backtracking, truncate the prefix to the current depth:
            self.buffer.truncate(depth);

            // Push the current node's children onto the stack:
            for (key, child) in &node.children {
                self.stack.push((child, depth + 1));
                self.buffer.push(key);

                // If the child node marks the end of a value, yield the current
                // prefix:
                if child.end_of_value {
                    // SAFETY: The prefix references keys in the trie, which
                    // have the same lifetime as the iterator.
                    return Some(
                        unsafe {
                            std::mem::transmute::<&[&T], &[T]>(
                                &self.buffer
                            )
                        }
                    );
                }
            }
        }
        None
    }
}
