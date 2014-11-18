extern crate getopts;
extern crate pkcs7pad;

use std::os;
use std::io::{stdin, File};
use getopts::{optopt, usage, getopts, optflag};
use pkcs7pad::pad;

#[allow(dead_code)]
fn main() {
    let opts = [optopt("s", "size", "block size (default 16)", "SIZE"),
                optflag("h", "help", "show usage")];

    let m = getopts(os::args().tail(), opts).ok().expect("Fail");

    if m.opt_present("h") {
        println!("{}", usage("Pad some text using PKCS#7 padding", opts));
        return;
    }

    let bsize = match m.opt_str("s") {
        Some(s) => from_str::<u8>(s.as_slice()).unwrap(),
        None => 16
    };

    let input: Vec<u8> = match m.free.as_slice() {
        [ref s, ..] => {
            let mut file = File::open(&Path::new(s));
            let input = file.read_to_end();
            input.ok().expect("Fail")},
        _ => stdin().read_to_end().ok().expect("Fail"),
    };

    let result = pad(input.as_slice(), bsize);

    print!("{}", result.into_ascii().into_string());
}
