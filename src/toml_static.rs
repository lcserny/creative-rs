use std::{env, fs};

use serde_derive::Deserialize;

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Redis {
    host: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Sqlite {
    db_file: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql,
}

pub fn read() {
    let config_const_values: Config = {
        let config_path = env::args().nth(1).unwrap();
        let config_text = fs::read_to_string(&config_path).unwrap();
        toml::from_str(&config_text).unwrap()
    };

    println!("Original: {:#?}", config_const_values);
    println!( "[Postgresql].Database: {}", config_const_values.postgresql.database );
}
