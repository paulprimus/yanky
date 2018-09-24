use std::io::{self, BufRead, Read, Write};
use std::sync::{mpsc};
use std::thread;

#[macro_use]
mod macros;
mod color;
mod screen;

//mod readinput;

const BLANK: &'static u8 = &b'\x20';
const LINE_BREAK: &'static u8 = &b'\x0a';

fn main() -> io::Result<()> {
    //let mut in_handle = stdin.lock();
    let (tx, rx) = mpsc::channel();
    // let mut stdin = io::stdin();
    read_user_input(tx.clone())?;
    read_stdin(tx.clone())?;

    let print_thread = thread::spawn(move || -> io::Result<()> {
        let mut stdout = io::stdout();
        for words in rx.iter() {
            for word in words {
                stdout.write(&word)?;
            }
        }
        Ok(())
    });
    match print_thread.join() {
        Ok(_) => (),
        Err(_) => ()
    };

    
    // let words: Vec<Vec<u8>> = rx.recv().unwrap();
    // for word in words {
    //     stdout.write(&word)?;
    // }

    Ok(())
}

fn read_user_input(tx: mpsc::Sender<Vec<Vec<u8>>>) -> io::Result<()> {
        thread::spawn(move || {
            println!(": ");
            let stdin = io::stdin();
            loop {
                let mut buffer = String::new();
                //let mut buffer: Vec<u8> = Vec::new();
                match stdin.read_line(&mut buffer) {
                    Ok(_) => (),
                    Err(e) => println!("Zeile konnte nicht gelesen werden: {}", e),
                };
                let mut words: Vec<Vec<u8>> = Vec::new();
                words.push(buffer.into_bytes());
                tx.send(words).unwrap();               
            }            
        });
    // });

    // for received in rx {
    //     println!("{}", received);
    // }
    Ok(())
}

fn read_stdin(tx: mpsc::Sender<Vec<Vec<u8>>>) -> io::Result<()> {
        thread::spawn(move || {
            let mut stdin = io::stdin();
            let mut buffer: Vec<u8> = Vec::new();
            match stdin.read_to_end(&mut buffer) {
                Ok(v) => println!("{}", v),
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
                    }
                    _ => {}
                };
            }
            tx.send(words).unwrap();
        });

    //});
    Ok(())
}

// fn write_stdout(all: &Vec<Vec<u8>>, highlighted: usize) -> io::Result<()> {
//     let stdout = io::stdout();
//     let mut out_handle = stdout.lock();
//     let mut count: usize = 0;
//     for word in all.iter() {
//         count = count + 1;
//         if highlighted == count {
//             out_handle.write(color::Gelb.as_ref())?;
//         }
//         match out_handle.write(word) {
//             Ok(_) => (),
//             Err(e) => println!("{}", e),
//         };
//         if highlighted == count {
//             out_handle.write(color::Standard.as_ref())?;
//         }
//     }
//     Ok(())
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
