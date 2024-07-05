use crate::tree::Tree;
use std::fmt;
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Tree<T>>,
}
impl<T> Node<T> {
    pub fn leaf(value: T) -> Self {
        Self {
            value,
            children: vec![Tree::Empty],
        }
    }
    pub fn new(value: T, children: Vec<Tree<T>>) -> Self {
        Self { value, children }
    }
    pub fn single(value: T, child: Tree<T>) -> Self {
        Self {
            value,
            children: vec![child],
        }
    }
}

impl<T> fmt::Display for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value: {}\n", &self.value)?;
        write!(f, "\tChildren:\n")?;
        for c in &self.children {
            write!(f, "\t{}", c)?;
        }
        Ok(())
    }
}
