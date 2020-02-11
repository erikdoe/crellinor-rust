extern crate crellinor;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut worldfile = None;
    if args.len() > 1 {
        worldfile = Some(&args[1]);
    }

    crellinor::run(worldfile);
    println!("Done. Reached the end of all worlds.");
}
