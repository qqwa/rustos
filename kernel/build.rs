use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("cargo:rerun-if-changed={}", "../config.toml");
    let mut conf_file = File::open("../config.toml").expect("could not find config file");

    let mut config = String::new();
    conf_file
        .read_to_string(&mut config)
        .expect("something went wrong reading the file");

    let config: toml::Value = toml::from_str(&config).unwrap();

    for (name, value) in config["config"].as_table().unwrap() {
        println!("cargo:rustc-cfg={}={}", name, value);
    }

    for (name, value) in config["feature"].as_table().unwrap() {
        if let Some(active) = value.as_bool() {
            if active {
                println!("cargo:rustc-cfg=feature=\"{}\"", name);
            }
        }
    }
}
