use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;
use std::string;


pub fn read_user_input() -> mpsc::Receiver<string::String> {
     let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        };
        tx.send(buffer).unwrap();
    });

    rx
}