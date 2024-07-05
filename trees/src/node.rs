use crate::tree::Tree;
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
