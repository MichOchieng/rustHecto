use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;

        if c.is_control(){
            println!("{:?}",b);
        } else {
            println!("{:?} ({})\r",b,c);
        }
        if b == ctrl('q') {
            break;
        }
    }
}
// Error handling: prints error and exits program
fn die(e: std::io::Error) {
    panic!(e)
}

// Used to exit program when control is pressed in combination with passed value
fn ctrl(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}
