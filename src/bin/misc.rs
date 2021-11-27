use log::{debug, error, info, warn};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!{
    static ref DICTIONARY: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(11, "food");
        m.insert(12, "bar");
        println!("Initialized");
        m
    };
}

fn main() {
    random_generations();
    logging();
    lazy_init();
}

fn random_generations() {
    let mut rng = thread_rng();
    println!("{}", rng.gen_range(0..20));
    println!("{}", rng.gen::<f64>());
    println!("{}", if rng.gen() { "Heads" } else { "Tails" });
}

fn logging() {
    env_logger::init();
    error!("Error message");
    warn!("Use RUST_LOG=debug env var to see this and below messages");
    info!("Info message");
    debug!("Debug message");
}

fn lazy_init() {
    println!("Started");
    println!("dict contains: {:?}", *DICTIONARY);
    println!("dict contains: {:?}", *DICTIONARY);
}