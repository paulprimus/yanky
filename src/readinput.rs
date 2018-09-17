use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::thread;
use std::string;


pub fn read_user_input() -> io::Result<()> {
     let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
       println!("Printing");
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("Zeile konnte nicht gelesen werden: {}", e),
        };
        tx.send(buffer).unwrap();
    });

    // let value= match rx.recv() {
    //     Ok(v) =>return  v,
    //     Err(e) => println!("Fehler: {}", e)
    // };
    let value = rx.recv().unwrap();
    let stdout = io::stdout();
    let mut out_handle= stdout.lock();
    out_handle.write(value.as_ref())?;
    Ok(())
}