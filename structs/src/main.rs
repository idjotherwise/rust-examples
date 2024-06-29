use env_logger::Env;
use log::info;

/// A FIFO queue
#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,   // older elements, eldest last.
    younger: Vec<char>, // younger elements, youngest last.
}

impl Queue {
    pub fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
    /// Push a character onto the back of a queue
    pub fn push(&mut self, c: char) {
        info!("Pushing {}", c);
        self.younger.push(c);
    }

    /// Pop a character from the queue
    pub fn pop(&mut self) -> Option<char> {
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

    let mut queue = Queue::new();
    let c = 'c';
    queue.push(c);
    if let Some(ch) = queue.pop() {
        info!("Took {} from queue", ch)
    }
    info!("{:?}", queue);
}
