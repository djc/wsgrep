extern crate regex;
#[macro_use]
extern crate structopt;

use regex::Regex;
use std::io::{self, BufRead, Write};
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    let re = Regex::new(&config.pattern).expect(&format!("invalid pattern {:?}", config.pattern));

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for ln in stdin.lock().lines() {
        let ln = ln.expect("error while reading line");
        if re.is_match(&ln) {
            handle.write(ln.as_bytes()).expect("error while writing line");
            handle.write(b"\n").expect("error while writing newline");
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "wsgrep", about = "workshop grep!")]
struct Config {
    pattern: String,
}
