use std::env;

fn main() {
    println!("cargo:rustc-cfg=device=\"{}\"", env::var("DEVICE").unwrap_or("imx8".to_string()));
}
