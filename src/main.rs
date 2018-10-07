use std::io::{self, Read, Write};
// use std::sync::{atomic, mpsc, Arc};
// use std::thread;

#[macro_use]
mod macros;
mod color;
mod screen;

//mod readinput;

const BLANK: &'static u8 = &b'\x20';
const LINE_BREAK: &'static u8 = &b'\x0a';

fn main() -> io::Result<()> {
    let input: Vec<Vec<u8>> = read_stdin()?;
    write_stdout(&input, 9usize)?;

    
    Ok(())
}

fn read_stdin() -> io::Result<Vec<Vec<u8>>> {
    let mut stdin = io::stdin();
    let mut buffer: Vec<u8> = Vec::new();

    match stdin.read_to_end(&mut buffer) {
        Ok(v) => println!("asdfasdf {}", v),
        Err(e) => println!("{}", e),
    };

    let mut word: Vec<u8> = Vec::new();
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
                // words = Vec::new();
            }
            _ => {}
        };
    }
    Ok(words)
}

fn write_stdout(all: &Vec<Vec<u8>>, highlighted: usize) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock();
    let mut count: usize = 0;
        out_handle.write(screen::Save.as_ref())?; 
    for word in all.iter() {
        count = count + 1;
        if highlighted == count {
            out_handle.write(color::Gelb.as_ref())?;
        }
        match out_handle.write(word) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };
        if highlighted == count {
            out_handle.write(color::Standard.as_ref())?;
        }
    }
    println!("test\n");  
    out_handle.write(screen::Restore.as_ref())?;  
    println!("test\n");  
   
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
