use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=.env");

    let dest_path = "./src/components/env.rs";
    let mut f = File::create(&dest_path).unwrap();

    // Load .env file
    dotenv().ok();

    // Write a dummy const, for example purposes
    f.write_all(b"pub const APP_WASM_FRAMEWORK: &'static str = \"FkPkN10hq7HCUJdK31YREnGXavKLyMALK9ovSFfU\";\n")
        .unwrap();

    // Write environment variables starting with "APP_"
    for (key, value) in env::vars() {
        if key.starts_with("APP_") {
            let line = format!(
                "pub const {}: &'static str = \"{}\";\n",
                key,
                value.replace("\"", "\\\"")
            );
            f.write_all(line.as_bytes()).unwrap();
        }
    }
}
