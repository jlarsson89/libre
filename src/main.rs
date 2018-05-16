extern crate getopts;
extern crate libusb;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _program = args[0].clone();
    let mut opts = Options::new();
    let context = libusb::Context::new().unwrap();
    for mut device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        println!("Bus {:03} Device {:03} ID: {:04x}:{:04x}",
                 device.bus_number(),
                 device.address(),
                 device_desc.vendor_id(),
                 device_desc.product_id());
    }
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
