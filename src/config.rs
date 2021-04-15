//! Read config into environment variable `SERVE_CONFIG`

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

use serde_json::{Value, from_str};

const CONFIG_FILE: &str = "serve_opt.json";
const CONFIG_ENV: &str = "SERVE_CONFIG";

/*
    TODO: Store config in cache
*/

/// read the config into the environment variable
pub fn setup_config() {
    let config_file = File::open(CONFIG_FILE).expect(
        &format!("Config file `{}` missing!", CONFIG_FILE)
    );
    let mut buf_reader = BufReader::new(config_file);
    let mut json_string = String::new();
    buf_reader.read_to_string(&mut json_string).unwrap();

    env::set_var(CONFIG_ENV, json_string);
}

/// returns the JSON config from the environment variable
pub fn get_config() -> Value {
    let bs_config = env::var(CONFIG_ENV).unwrap();

    let json_config: Value = from_str(&bs_config).expect(
        &format!("Config file `{}` not well-formatted!", CONFIG_FILE)
    );

    json_config
}
