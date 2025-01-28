use crate::node::TrieNode;

pub struct StringTrieIter<'a, H> {
    stack: Vec<(&'a TrieNode<char, H>, usize)>,
    buffer: String,
}

impl<'a, H> StringTrieIter<'a, H> {
    pub fn new(root: &'a TrieNode<char, H>) -> Self {
        Self {
            stack: vec![(root, 0)],
            buffer: String::new(),
        }
    }
}

impl<H> Iterator for StringTrieIter<'_, H> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Dequeue the stack:
        while let Some((node, depth)) = self.stack.pop() {
            // If we're backtracking, truncate the prefix to the current depth:
            self.buffer.truncate(depth);

            // Push the current node's children onto the stack:
            for (key, child) in &node.children {
                self.stack.push((child, depth + 1));
                self.buffer.push(*key);

                // End of value reached, return the buffer:
                if child.end_of_value {
                    return Some(self.buffer.to_string())
                }
            }
        }
        None
    }
}