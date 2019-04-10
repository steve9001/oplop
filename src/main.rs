use std::env;
use oplop;

fn main() {
    let args: Vec<String> = env::args().collect();
    oplop::run(&args[1..]);
}
