use log::info;
use std::fmt;

use crate::node::Node;

// TODO: implement show Display trait

#[derive(Debug, Clone)]
pub enum Tree<T> {
    Empty,
    // Node which may have children
    NonEmpty(Box<Node<T>>),
    // Might need to have a leaf variant to distinguish from non-leafs
}

impl<T> Tree<T> {
    fn is_tree(&self) -> bool {
        match self {
            Tree::Empty => false,
            _ => true,
        }
    }
    pub fn from_node(node: Node<T>) -> Self {
        Tree::NonEmpty(Box::new(node))
    }
}

impl<T> fmt::Display for Tree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "Empty")
            }
            Self::NonEmpty(n) => {
                write!(f, "{:4}", n)
            }
        }
    }
}
