pub mod trie;
pub mod node;

#[cfg(feature = "string")]
pub mod string;

#[allow(missing_docs)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        trie::Trie,
        node::TrieNode,
    };

    #[doc(hidden)]
    #[cfg(feature = "string")]
    pub use crate::string::StringTrie;
}