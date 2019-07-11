use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{self, BufRead, BufReader, LineWriter, Write};
use std::path::PathBuf;

use regex::Regex;
use structopt::StructOpt;
use termion::color;

fn main() -> Result<(), Error> {
    let config = Config::from_args();
    let re = Regex::new(&config.pattern)?;
    let stdout = io::stdout();
    let mut out = stdout.lock();

    match config.input {
        Some(file) => process(BufReader::new(File::open(file)?), re, &mut out),
        None => process(io::stdin().lock(), re, &mut out),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "wsgrep", about = "workshop grep!")]
struct Config {
    pattern: String,
    #[structopt(short = "i", parse(from_os_str))]
    input: Option<PathBuf>,
}

fn process<I, O>(iter: I, re: Regex, handle: &mut O) -> Result<(), Error>
where
    I: BufRead,
    O: Write,
{
    let mut handle = LineWriter::new(handle);
    let mut hl_start = String::new();
    write!(hl_start, "{}", color::Fg(color::Red)).unwrap();
    let mut hl_end = String::new();
    write!(hl_end, "{}", color::Fg(color::Reset)).unwrap();

    let mut hl = false;
    for ln in iter.lines() {
        let ln = ln?;
        let mut cur = 0;

        let mut matched = false;
        for m in re.find_iter(&ln) {
            let start = m.start();
            let end = m.end();
            matched = true;

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

        if matched {
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

    Ok(())
}

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Re(regex::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Error {
        Error::Re(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = "foofds-fdsfoo".as_bytes();
        let mut out = Vec::new();
        let re = Regex::new("fds").unwrap();
        process(input, re, &mut out).unwrap();
        assert_eq!(out.as_slice(), &b"foo\x1b[38;5;1mfds\x1b[39m-\x1b[38;5;1mfds\x1b[39mfoo\n"[..]);
    }

    #[test]
    fn no_match() {
        let input = "foofds-fdsfoo".as_bytes();
        let mut out = Vec::new();
        let re = Regex::new("aw").unwrap();
        process(input, re, &mut out).unwrap();
        assert_eq!(out.as_slice(), &b""[..]);
    }
}
