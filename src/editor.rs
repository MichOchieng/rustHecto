use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor{
    quit: bool,
    terminal: Terminal,
}

impl Editor {

    pub fn run(&mut self) {

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.quit{
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        Self{
            quit: false,
            terminal: Terminal::default().expect("Failed to init terminal!"),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_postion(0,0);
        if self.quit{
            Terminal::clear_screen();
            println!("Cya!\r");
        }
        else {
            self.draw_rows();
            Terminal::cursor_postion(0,0);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for row in 0..self.terminal.size().height{
            let height = self.terminal.size().height;
            Terminal::clear_ln();
            if row == height / 3 {
                self.draw_welcome();
            } else {
                println!("~\r");
            }
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_welcome(&self) {
        let mut welcome = format!("Hecto editor -- V{}\r",VERSION);
        let width       = self.terminal.size().width as usize;
        let len         = welcome.len();
        let padding     = width.saturating_sub(len) / 2;
        let spaces      = " ".repeat(padding.saturating_sub(1));
        welcome         = format!("~{}{}",spaces,welcome);
        welcome.truncate(width);
        println!("{}\r", welcome);
    }
}

// Error handling: prints error and exits program
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}",e);
}