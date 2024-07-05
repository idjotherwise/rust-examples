use env_logger::Env;
use log::info;
use node::Node;
use tree::Tree;

mod node;
mod tree;

fn main() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let node11 = Node::leaf("Node1");

    let node1 = Node::single("Node2", Tree::from_node(node11));
    let node2 = Node {
        value: "Node2",
        children: vec![Tree::from_node(node1.clone())],
    };
    let node3 = Node {
        value: "Node3",
        children: vec![Tree::from_node(node1.clone()), Tree::from_node(node2)],
    };

    let tree = Tree::NonEmpty(Box::new(node3));

    println!("{:#?}", tree);
}
