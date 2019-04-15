use oplop;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    oplop::run(&args[1..]);
}
