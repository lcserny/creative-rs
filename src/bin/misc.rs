use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    fmt::{self, Result},
    path::PathBuf,
    result,
};
use structopt::StructOpt;

lazy_static! {
    static ref DICTIONARY: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(11, "food");
        m.insert(12, "bar");
        println!("Initialized");
        m
    };
    static ref NUMBERS: Vec<u32> = {
        let mut n = Vec::new();
        for i in 1..201 {
            n.push(i * i);
        }
        n
    };
}

fn main() {
    random_generations();
    logging();
    lazy_init();
    parse_args();
}

// cargo run -- myfile1.txt -v --result hello.txt --level 1
#[derive(StructOpt, Debug)]
struct Opts {
    /// Activate verbose mode
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// File to generate
    #[structopt(short = "r", long = "result", parse(from_os_str))]
    result_file: PathBuf,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,

    #[structopt(short = "l", long = "level", parse(try_from_str = parse_range))]
    level: u8,
}

#[derive(Debug, Clone)]
struct MyError {
    src: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        write!(f, "invalid number, expected between 1 and 20, provided: {}", self.src)
    }
}

fn parse_range(src: &str) -> result::Result<u8, MyError> {
    if let Ok(n) = src.parse::<u8>() {
        if n >= 1 && n <= 20 {
            return Ok(n);
        }
    }
    Err(MyError { src: String::from(src), })
}

fn parse_args() {
    println!("{:#?}", Opts::from_args());
}

fn random_generations() {
    let mut rng = thread_rng();
    println!("{}", rng.gen_range(0..20));
    println!("{}", rng.gen::<f64>());
    println!("{}", if rng.gen() { "Heads" } else { "Tails" });

    for _ in 0..10 {
        let num: f32 = rng.gen_range(100f32..400f32);
        println!("{}", num);
    }
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

    println!("{:?}", *NUMBERS);
}
