use std::env;
use minigrep_rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    
    minigrep_rust::run(config);
}
