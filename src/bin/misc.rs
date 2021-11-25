use rand::{Rng, thread_rng};

fn main() {
    random_generations();
}

fn random_generations() {
    let mut rng = thread_rng();
    println!("{}", rng.gen_range(0..20));
    println!("{}", rng.gen::<f64>());
    println!("{}", if rng.gen() { "Heads" } else { "Tails" });
}