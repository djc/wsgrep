extern crate regex;
#[macro_use]
extern crate structopt;
extern crate termion;

use regex::Regex;
use std::fmt::Write as FmtWrite;
use std::io::{self, BufRead, Write};
use structopt::StructOpt;
use termion::color;

fn main() {
    let config = Config::from_args();
    let re = Regex::new(&config.pattern).expect(&format!("invalid pattern {:?}", config.pattern));

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut hl_start = String::new();
    write!(hl_start, "{}", color::Fg(color::Red)).unwrap();
    let mut hl_end = String::new();
    write!(hl_end, "{}", color::Fg(color::Reset)).unwrap();

    for ln in stdin.lock().lines() {
        let ln = ln.expect("error while reading line");
        let m = match re.find(&ln) {
            Some(m) => m,
            None => continue,
        };

        let start = m.start();
        let end = m.end();
        handle.write(ln[..start].as_bytes()).unwrap();

        handle.write(&hl_start.as_bytes()).unwrap();
        handle.write(ln[start..end].as_bytes()).unwrap();
        handle.write(&hl_end.as_bytes()).unwrap();

        handle.write(ln[end..].as_bytes()).unwrap();
        handle.write(b"\n").unwrap();
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "wsgrep", about = "workshop grep!")]
struct Config {
    pattern: String,
}
