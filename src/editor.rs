use crate::Terminal;
use termion::event::Key;

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
            quit: false
            temrinal: Terminal::default().expect("Failed to init terminal!").
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_postion(0,0);
        if self.quit{
            println!("Cya!\r");
        }
        else {
            self.draw_rows();
            Termianl::cursor_postion(0,0);
        }
        Terminal::flush()
    }

    fn draw_rows(&self) {
        for _ in 0..self.terminal.size().height{
            println!("~\r");
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
}

// Error handling: prints error and exits program
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}",e);
}