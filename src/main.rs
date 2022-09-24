extern crate crellinor;

use std::env;
use std::process::exit;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("f", "file", "Load world from file. If you use a log file the simulation will restart from the beginning.", "PATH");
    opts.optflag("w", "web", "Run the web server. A world file must be given.");
    opts.optflag("h", "help", "Display this help message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            print!("{}", opts.usage(&f.to_string()));
            exit(-1);
        }
    };
    if matches.opt_present("help") {
        print!("{}", opts.usage(&"Usage: crellinor [OPTIONS]"));
        exit(0);
    }

    let worldfile = matches.opt_str("file");
    let run_web = matches.opt_present("web");

    crellinor::run(worldfile, run_web);
}
