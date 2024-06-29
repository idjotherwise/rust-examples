use crate::queue::Queue;

mod lifetimes;
mod queue;

use env_logger::Env;
use lifetimes::{find_extrema, Extrema};
use log::info;

fn main() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    // Generic Queue
    let mut queue = Queue::<char>::new();
    let c = 'c';
    queue.push(c);
    if let Some(ch) = queue.pop() {
        info!("Took {} from queue", ch)
    }
    info!("{:?}", queue);

    // Lifetime
    let a = [0, -3, 0, 15, 49];
    info!("Finding extrema for: {:?}", a);
    let extrema: Extrema = find_extrema(&a);
    info!("Max: {}, Min: {}", extrema.greatest, extrema.least);
}
