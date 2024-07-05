use log::info;

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
