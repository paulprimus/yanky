use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let stdin = io::stdin();
    let mut in_handle = stdin.lock();

    let stdout = io::stdout();
    let mut out_handle = stdout.lock();

    match in_handle.read_to_end(&mut buffer) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    };

    let mut word: Vec<u8> = Vec::new();
    let mut all: Vec<Vec<u8>> = Vec::new();
    for i in &buffer {
       match i {
           b'\x20' => {
                all.push(word);
                word = Vec::new();
            },    
           _ => {
                    word.push(*i);
                },     
       };
    }

    for i in &all {
    match out_handle.write(&i) {
        Ok(_) =>  continue,
        Err(e) => println!("{}", e),
    };
    }
    Ok(())
}
