#![feature(slice_patterns)]

extern crate getopts;
extern crate pkcs7pad;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::stdin;
use getopts::Options;
//{optopt, usage, getopts, optflag};
use pkcs7pad::{pad, unpad, validate_padding};
use std::str::FromStr;

enum Operation {
    Pad,
    Unpad,
    Validate,
}

#[allow(dead_code)]
fn main() {
    let mut opts = Options::new();

    opts.optopt("s", "size", "block size (default 16)", "SIZE");
    opts.optflag("h", "help", "show usage");
    opts.optflag("p", "pad", "pad");
    opts.optflag("u", "unpad", "unpad");
    opts.optflag("", "validate", "validate");

    let args: Vec<String> = std::env::args().collect();
    let m = opts.parse(&args[1..]).ok().expect("Fail");

    if m.opt_present("h") {
        println!("{}", opts.usage("Pad some text using PKCS#7 padding"));
        return;
    }

    let bsize = match m.opt_str("s") {
        Some(s) => u8::from_str(&s).unwrap(),
        None => 16u8
    };

    let input: Vec<u8> = match m.free.as_slice() {
        [ref s, ..] => {
            let mut buf = Vec::new();
            let file = File::open(&Path::new(s));
            let mut input = file.unwrap();
            input.read_to_end(&mut buf).unwrap();
            buf
        },
        _ => {
            let mut buf = Vec::new();
            stdin().read_to_end(&mut buf).unwrap();
            buf
        }
    };

    let operation =
        if m.opt_present("validate") {
            Operation::Validate
        } else if m.opt_present("unpad") {
            Operation::Unpad
        } else {
            Operation::Pad
        };

    match operation {
        Operation::Pad => {
            let result = pad(input.as_slice(), bsize);
            print!("{}", String::from_utf8(result).unwrap());},
        Operation::Unpad => {
            let result = unpad(input.as_slice()).unwrap();
            print!("{}", String::from_utf8(result).unwrap());},
        Operation::Validate => {
            if validate_padding(input.as_slice()) {
                std::process::exit(1);}},
    };
}
