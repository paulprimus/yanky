#[cfg(unix)]
extern crate termion;

use std::io::{self, Read, Write};

const BLANK: &'static u8 = &b'\x20';
const LINE_BREAK: &'static u8 = &b'\x0a';

fn main() -> io::Result<()> {
    let all: Vec<Vec<Vec<u8>>> = read_stdin();

    write_stdout(&all);
    Ok(())
}

fn read_stdin() -> Vec<Vec<Vec<u8>>> {
    let mut buffer: Vec<u8> = Vec::new();
    let stdin = io::stdin();
    let mut in_handle = stdin.lock();
    let save = termion::cursor::Save;
    match in_handle.read_to_end(&mut buffer) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    };

    let mut word: Vec<u8> = Vec::new();
    let mut line: Vec<Vec<u8>> = Vec::new();
    let mut all: Vec<Vec<Vec<u8>>> = Vec::new();
    for i in &buffer {
        word.push(*i);
        match i {
            BLANK => {
                line.push(word);
                word = Vec::new();
            }
            LINE_BREAK => {
                all.push(line);
                line = Vec::new();
            }
            _ => {

                //     word.push(*i);
            }
        };
    }
    line.push(word);
    all.push(line);
    return all;
}

fn write_stdout(all: &Vec<Vec<Vec<u8>>>) {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock();

    for line in all {
        for word in line {
            match out_handle.write(&word) {
                Ok(_) => continue,
                Err(e) => println!("{}", e),
            };
        }
    }

     write!(out_handle,"{}",termion::cursor::Restore);
}
