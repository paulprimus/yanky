use std::io::{Write, Read, stdin, stdout};
use std::str;

fn main() {
    let stdout = stdout();
    let  mut stdout_lock = stdout.lock();
    let stdin = stdin();
    let mut buffer: Vec<u8> = Vec::new();
    //let mut buffer: [u8; 500] = [0; 500];

    let mut handle = stdin.lock();
    match handle.read_to_end(&mut buffer) {
        Ok(n) => println!("{}", n),
        Err(_err) => println!("Fehler: {}", _err),
    }

    let s = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(err) => panic!("Invalid UTF-8 sequence: {}", err),
    };
    println!("{}\n\n",s );

   stdout_lock.write_all(&buffer).unwrap();
    //}
}
