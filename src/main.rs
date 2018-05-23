use std::io::{self, Read, Write};
use std::time::{SystemTime, Duration};

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


/// Types that allow detection of the cursor position.
pub trait DetectCursorPos {
    /// Get the (1,1)-based cursor position from the terminal.
    fn cursor_pos(&mut self) -> io::Result<(u16, u16)>;
}

impl<W: Write> DetectCursorPos for W {
    fn cursor_pos(&mut self) -> io::Result<(u16, u16)> {
        let delimiter = b'R';
        let mut stdin = async_stdin_until(delimiter);

        // Where is the cursor?
        // Use `ESC [ 6 n`.
        write!(self, "\x1B[6n")?;
        self.flush()?;

        let mut buf: [u8; 1] = [0];
        let mut read_chars = Vec::new();

        let timeout = Duration::from_millis(CONTROL_SEQUENCE_TIMEOUT);
        let now = SystemTime::now();

        // Either consume all data up to R or wait for a timeout.
        while buf[0] != delimiter && now.elapsed().unwrap() < timeout {
            if stdin.read(&mut buf)? > 0 {
                read_chars.push(buf[0]);
            }
        }

        if read_chars.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "Cursor position detection timed out."));
        }

        // The answer will look like `ESC [ Cy ; Cx R`.

        read_chars.pop(); // remove trailing R.
        let read_str = String::from_utf8(read_chars).unwrap();
        let beg = read_str.rfind('[').unwrap();
        let coords: String = read_str.chars().skip(beg + 1).collect();
        let mut nums = coords.split(';');

        let cy = nums.next()
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let cx = nums.next()
            .unwrap()
            .parse::<u16>()
            .unwrap();

        Ok((cx, cy))
    }
}