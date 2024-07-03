use env_logger::Env;
use log::info;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    // Hours,
    // Days,
    // Months,
    // Years,
}
impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            // TimeUnit::Hours => "hours",
            // TimeUnit::Days => "days",
            // TimeUnit::Months => "months",
            // TimeUnit::Years => "years",
        }
    }
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    // JustNow,
    // InThefuture(TimeUnit, u32),
}

fn main() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);
    let rt = RoughTime::InThePast(TimeUnit::Seconds, 1);
    let tu = TimeUnit::Minutes;
    let tup = tu.plural();
    let tus = tu.singular();
    println!("{:?} {:?} {:?} {:?}", rt, tu, tup, tus);

    let tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));
}

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>), // just a pointer to a hashmap
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}
