use std::io::{self, Read, Write};
use std::sync::{atomic, mpsc, Arc};
use std::thread;

#[macro_use]
mod macros;
mod color;
mod screen;

//mod readinput;

const BLANK: &'static u8 = &b'\x20';
const LINE_BREAK: &'static u8 = &b'\x0a';

struct input {
    words: Vec<Vec<u8>>,
}

fn main() -> io::Result<()> {
    let i = read_stdin().unwrap();
    write_stdout(&i, 9usize);

    // read_user_input().join();
    //read_stdin(tx.clone())?;

    // let print_thread = thread::spawn(move || -> io::Result<()> {

    //     Ok(())
    // });
    // match print_thread.join() {
    //     Ok(_) => (),
    //     Err(_) => (),
    // };

    // let words: Vec<Vec<u8>> = rx.recv().unwrap();
    // for word in words {
    //     stdout.write(&word)?;
    // }

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

// fn read_user_input() -> thread::JoinHandle<()> {
//     thread::spawn(move || {
//         println!(": ");
//         let stdin = io::stdin();
//         let mut stdout = io::stdout();
//         let mut done = false;
//         while !done  {
//             let mut buffer = String::new();
//             //let mut buffer: Vec<u8> = Vec::new();
//             match stdin.read_line(&mut buffer) {
//                 Ok(size) => {
//                     if size == 0 {
//                      done = true;
//                     }
//                 },
//                 Err(e) => println!("Zeile konnte nicht gelesen werden: {}", e),
//             };
//             if buffer.trim() == "exit" || done  {
//                 println!("Goodbye");
//                 done = true;
//             } else {
//                 stdout.write(buffer.as_bytes());
//             //    words.push(buffer.into_bytes());
//             }
//         }
//     })
//     // });

//     // for received in rx {
//     //     println!("{}", received);
//     // }
//     // Ok(())
// }

fn write_stdout(all: &Vec<Vec<u8>>, highlighted: usize) -> io::Result<()> {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock();
    let mut count: usize = 0;
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
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
