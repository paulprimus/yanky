use std::io::{self, Read, Write};

fn main() -> io::Result<()> {

    let all: Vec<Vec<Vec<u8>>> = read_stdin();
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
        match i {
            b'\x20' => { // space
                let mut blank: Vec<u8> = Vec::new();
                blank.push(b'\x25');
                line.push(blank);
                line.push(word);
                word = Vec::new();                
            }
            b'\x0a' => { // linefeed              
                all.push(line);
                line = Vec::new();
            }
            _ => {
                word.push(*i);
            }
        };
    }
    return all;
}

fn write_stdout(all: & Vec<Vec<Vec<u8>>>) {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock(); 

    for line in all {
        for word in line {
        match out_handle.write(&word) {
            Ok(_) => continue,
            Err(e) => println!("{}", e),
        };
        }
    }
}
