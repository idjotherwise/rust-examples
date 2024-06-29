use crate::queue::Queue;

mod lifetimes;
mod polynomial;
mod queue;

use env_logger::Env;
use lifetimes::{find_extrema, Extrema};
use log::info;
use polynomial::Polynomial;

fn main() {
    let env = Env::default().filter_or("LOG_LEVEL", "info");

    env_logger::init_from_env(env);

    // - Queues
    // Type parameter generics
    let mut queue = Queue::<char>::new();
    let c = 'c';
    queue.push(c);
    if let Some(ch) = queue.pop() {
        info!(target: "Queues", "Took {} from queue", ch)
    }
    info!("{:?}", queue);

    // Lifetimes parameter generics
    let a = [0, -3, 0, 15, 49];
    info!(target: "Lifetimes", "Finding extrema for: {:?}", a);
    let extrema: Extrema = find_extrema(&a);
    info!(target: "Lifetimes", "Max: {}, Min: {}", extrema.greatest, extrema.least);

    // - Polynomials
    // Const parameter generics
    let coeffs = [1., 5., 3., -2., 10., 0., 1.];
    let poly = Polynomial::new(coeffs);
    let val = poly.eval(1.);
    info!(target: "Const params", "Polynomial evaluated at 1: {}", val);
}
