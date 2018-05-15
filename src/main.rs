extern crate getopts;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("r", "read", "read data");
    opts.optflag("e", "export", "export data");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("r") {
        println!("read");
        return;
    }
    if matches.opt_present("e") {
        println!("export");
        return;
    }
}
