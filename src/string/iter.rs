use crate::node::TrieNode;

pub struct StringTrieIter<'a, H> {
    stack: Vec<(char, &'a TrieNode<char, H>, usize)>,
    buffer: String,
}

impl<'a, H> StringTrieIter<'a, H> {
    pub fn new(root: &'a TrieNode<char, H>) -> Self {
        let mut stack = Vec::with_capacity(
            root.children.len()
        );
        for (key, child) in &root.children {
            stack.push((*key, child, 0));
        }
        Self {
            stack,
            buffer: String::new(),
        }
    }
}

impl<H> Iterator for StringTrieIter<'_, H> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Dequeue the stack:
        while let Some((key, node, depth)) = self.stack.pop() {
            // If we're backtracking, truncate the prefix to the current depth:
            self.buffer.truncate(depth);
            self.buffer.push(key);

            // End of value reached, return the buffer:
            if node.end_of_value {
                return Some(self.buffer.to_string());
            }

            // Push the current node's children onto the stack:
            for (key, child) in &node.children {
                self.stack.push((*key, child, depth + 1));
            }
        }
        None
    }
}