
use std::io::{self, Read, Write};

#[macro_use]
mod macros;
mod screen;
mod color;

const BLANK: &'static u8 = &b'\x20';
const LINE_BREAK: &'static u8 = &b'\x0a';

fn main() -> io::Result<()> {
    let all: Vec<Vec<Vec<u8>>> = read_stdin();
    write_stdout(&all)?;
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
            BLANK => {
                line.push(word);
                word = Vec::new();
            }
            LINE_BREAK => {
                all.push(line);
                line = Vec::new();
            }
            _ => {
            }
        };
    }
    line.push(word);
    all.push(line);
    return all;
}

fn write_stdout(all: &Vec<Vec<Vec<u8>>>) -> io::Result<()>  {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock();   
    out_handle.write(color::Gelb.as_ref())?;

    for line in all {
        for word in line {
            match out_handle.write(&word) {
                Ok(_) => continue,
                Err(e) => println!("{}", e),
            };
        }
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