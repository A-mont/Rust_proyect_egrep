
use std::env;
use Rust_project_minigrep::Config;

        
fn main() {


    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("File: {}", config.filename);
    println!("Search: {}",config.query);


    Rust_project_minigrep::run(config);


}