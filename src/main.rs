extern crate getopts;
use getopts::Options;
use std::env;

pub mod reader;
pub mod export;

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
        reader::read_data();
        return;
    }
    if matches.opt_present("e") {
        export::export_data();
        return;
    }
}
