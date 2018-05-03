extern crate regex;
#[macro_use]
extern crate structopt;
extern crate termion;

use regex::Regex;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use termion::color;

fn main() {
    let config = Config::from_args();
    let re = Regex::new(&config.pattern).expect(&format!("invalid pattern {:?}", config.pattern));
    match config.input {
        Some(file) => {
            let f = File::open(file).unwrap();
            process(BufReader::new(f), re);
        }
        None => {
            let stdin = io::stdin();
            process(stdin.lock(), re);
        }
    }
}

fn process<T>(iter: T, re: Regex)
where
    T: BufRead,
{
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut hl_start = String::new();
    write!(hl_start, "{}", color::Fg(color::Red)).unwrap();
    let mut hl_end = String::new();
    write!(hl_end, "{}", color::Fg(color::Reset)).unwrap();

    let mut hl = false;
    for ln in iter.lines() {
        let ln = ln.expect("error while reading line");
        let mut cur = 0;

        for m in re.find_iter(&ln) {
            let start = m.start();
            let end = m.end();

            if start > cur {
                if hl {
                    handle.write(&hl_end.as_bytes()).unwrap();
                    hl = false;
                }
                handle.write(ln[cur..start].as_bytes()).unwrap();
            }

            if !hl {
                handle.write(&hl_start.as_bytes()).unwrap();
                hl = true;
            }
            handle.write(ln[start..end].as_bytes()).unwrap();
            cur = end;
        }

        if cur < ln.len() {
            if hl {
                handle.write(&hl_end.as_bytes()).unwrap();
                hl = false;
            }
            handle.write(ln[cur..].as_bytes()).unwrap();
        }
        handle.write(b"\n").unwrap();
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "wsgrep", about = "workshop grep!")]
struct Config {
    pattern: String,
    #[structopt(short = "i", parse(from_os_str))]
    input: Option<PathBuf>,
}
