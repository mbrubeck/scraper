//! HTML documents and fragments.

use std::borrow::Cow;

use ego_tree::Tree;
use ego_tree::iter::Nodes;
use html5ever::driver;
use html5ever::tree_builder::QuirksMode;
use tendril::StrTendril;

use node::Node;
use node_ref::NodeRef;
use selector::Selector;

/// An HTML tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Html {
    /// Parse errors.
    pub errors: Vec<Cow<'static, str>>,

    /// The quirks mode.
    pub quirks_mode: QuirksMode,

    /// The node tree.
    pub tree: Tree<Node>,
}

impl Html {
    /// Creates an empty HTML document.
    pub fn new_document() -> Self {
        Html {
            errors: Vec::new(),
            quirks_mode: QuirksMode::NoQuirks,
            tree: Tree::new(Node::Document),
        }
    }

    /// Creates an empty HTML fragment.
    pub fn new_fragment() -> Self {
        Html {
            errors: Vec::new(),
            quirks_mode: QuirksMode::NoQuirks,
            tree: Tree::new(Node::Fragment),
        }
    }

    /// Parses a string of HTML as a document.
    pub fn parse_document(document: &str) -> Self {
        driver::parse_to(
            Self::new_document(),
            driver::one_input(StrTendril::from_slice(document)),
            Default::default()
        )
    }

    /// Parses a string of HTML as a fragment.
    pub fn parse_fragment(fragment: &str) -> Self {
        driver::parse_fragment_to(
            Self::new_fragment(),
            driver::one_input(StrTendril::from_slice(fragment)),
            qualname!(html, "body"),
            Vec::new(),
            Default::default()
        )
    }

    /// Returns an iterator over elements matching a selector.
    pub fn select<'a, 'b>(&'a self, selector: &'b Selector) -> Select<'a, 'b> {
        Select {
            selector: selector,
            inner: self.tree.nodes(),
        }
    }
}

/// Iterator over elements matching a selector.
#[derive(Debug)]
pub struct Select<'a, 'b> {
    inner: Nodes<'a, Node>,
    selector: &'b Selector,
}

impl<'a, 'b> Iterator for Select<'a, 'b> {
    type Item = NodeRef<'a>;

    fn next(&mut self) -> Option<NodeRef<'a>> {
        for node in self.inner.by_ref() {
            let node_ref = NodeRef(node);
            if node.value().is_element() && self.selector.matches(&node_ref) {
                return Some(node_ref);
            }
        }
        None
    }
}

mod tree_sink;