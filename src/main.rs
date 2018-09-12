use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::fmt;

#[macro_use]
mod macros;
mod color;
mod screen;

const BLANK: &'static u8 = &b'\x20';
const LINE_BREAK: &'static u8 = &b'\x0a';

fn main() -> io::Result<()> {
    let all: Vec<Vec<u8>> = read_stdin();
    write_stdout(&all, 2u32)?;
    Ok(())
}

fn read_stdin() -> Vec<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();
    let stdin = io::stdin();
    let mut in_handle = stdin.lock();

    match in_handle.read_to_end(&mut buffer) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    };

    let mut word: Vec<u8> = Vec::new();
    // let mut line: Vec<Vec<u8>> = Vec::new();
    // let mut all: Vec<Vec<Vec<u8>>> = Vec::new();

    let mut words: Vec<Vec<u8>> = Vec::new();

    for v in &buffer {
        word.push(*v);
        match v {
            BLANK => {
                words.push(word);
                word = Vec::new();
            }
            LINE_BREAK => {
                words.push(word);
                word = Vec::new();
            }
            _ => {}
        };
    }
    return words;
}

fn write_stdout(all: &Vec<Vec<u8>>, highlighted: usize) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock();
    // out_handle.write(color::Gelb.as_ref())?;

    // for line in all {
    //     for word in line {
    //         match out_handle.write(&word) {
    //             Ok(_) => continue,
    //             Err(e) => println!("{}", e),
    //         };
    //     }
    // }

    let word: Vec<u8> = all[highlighted];
    let new_word: Vec<u8> = Vec::new();
    for v in color::Gelb.as_ref() {
        new_word.push(v);
    }


    for word in all.iter() {
        
        match out_handle.write(word) {
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        };
       
    }

    out_handle.write(screen::Reset.as_ref())?;
    out_handle.write(screen::DeleteLine.as_ref())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
