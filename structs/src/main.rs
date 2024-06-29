use env_logger::Env;
use log::info;
use std::fmt::Debug;

/// A FIFO queue
#[derive(Debug)]
pub struct Queue<T> {
    older: Vec<T>,   // older elements, eldest last.
    younger: Vec<T>, // younger elements, youngest last.
}

impl<T> Queue<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
    /// Push a character onto the back of a queue
    pub fn push(&mut self, c: T) {
        info!("Pushing {:?}", c);
        self.younger.push(c);
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    /// Pop a character from the queue
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring the elements in younger over to older, and put them in the promised order
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            // Need to reverse since older and younger are flipped orders
            self.older.reverse();
        }

        // Now older is guaranteed to not be empty, so we can pop to get an option
        self.older.pop()
    }
}

fn main() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    let mut queue = Queue::<char>::new();
    let c = 'c';
    queue.push(c);
    if let Some(ch) = queue.pop() {
        info!("Took {} from queue", ch)
    }
    info!("{:?}", queue);
}
