#[cfg(unix)]
extern crate termion;

use std::io::{self, Read, Write};
use termion::cursor;

const BLANK: &'static u8 = &b'\x20';
const LINE_BREAK: &'static u8 = &b'\x0a';


fn main() -> io::Result<()> {

    let all: Vec<Vec<Vec<u8>>> = read_stdin();
    get_cursor_pos();
    write_stdout(&all);
    Ok(())
}

fn read_stdin() -> Vec<Vec<Vec<u8>>> {
    let mut buffer: Vec<u8> = Vec::new();
    let stdin = io::stdin();
    let mut in_handle = stdin.lock();

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
            BLANK => { // space
                // let mut blank_vec: Vec<u8> = Vec::new();
                // blank_vec.push(*BLANK);
                // line.push(blank_vec);
                line.push(word);
                word = Vec::new();                
            }
            LINE_BREAK => { // linebreak  
                // let mut line_break: Vec<u8> = Vec::new();
                // line_break.push(*LINE_BREAK);
                // line.push(line_break);                       
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

fn write_stdout(all: & Vec<Vec<Vec<u8>>>) {
    let  stdout = io::stdout();
    let mut out_handle = stdout.lock(); 
    cursor::Goto(4, 2);
   // write!(out_handle,"{}", cursor::Goto(4, 2));
    for line in all {
        for word in line {
        match out_handle.write(&word) {
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        };
        }
    }
    out_handle.write(cursor::Goto(4, 2));
}

// #[cfg(unix)]
fn get_cursor_pos() {
    //println!("fuck");
    cursor::Goto(4, 2);
}
