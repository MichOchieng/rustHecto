use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = stdout().into_raw_mode().unwrap();

    for key in io::stdin().keys() {
    // If wrapped b is Ok do something otherwise Err
       match key {
        //    Avoiding into_raw_mode error
        //    Handling output
           Ok(key) => match key {
               Key::Char(c) => {
                   if c.is_control(){
                       println!("{:?}\r",c as u8);
                   }else {
                    println!("{:?} ({})\r",c as u8,c);
                   }
                }
               Key::Ctrl('q') => break,
               _ => println!("{:?}\r",key),
           },
           Err(err) => die(err),
       }
    }
}

// Error handling: prints error and exits program
fn die(e: std::io::Error) {
    panic!("{}",e)
}
