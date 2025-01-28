use crate::node::TrieNode;

/// Iterates over a [`TrieNode`].
pub struct TrieIter<'a, T, H> {
    stack: Vec<(T, &'a TrieNode<T, H>, usize)>,
    buffer: Vec<T>,
}

impl<'a, T, H> TrieIter<'a, T, H>
where
    T: Copy,
{
    /// Creates a new [`TrieIter`] over a root [`TrieNode`].
    pub fn new(root: &'a TrieNode<T, H>) -> Self {
        let mut stack = Vec::with_capacity(
            root.children.len()
        );
        for (key, child) in &root.children {
            stack.push((*key, child, 0));
        }
        Self {
            stack,
            buffer: Vec::new(),
        }
    }
}

impl<T, H> Iterator for TrieIter<'_, T, H>
where
    T: Eq + Copy,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((key, node, depth)) = self.stack.pop() {
            // If we're backtracking, truncate the prefix to the current depth:
            self.buffer.truncate(depth);
            self.buffer.push(key);

            if node.end_of_value {
                return Some(self.buffer.clone());
            }

            for (key, child) in &node.children {
                self.stack.push((*key, child, depth + 1));
            }
        }
        None
    }
}
