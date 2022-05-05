use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor{
    quit: bool,
    terminal: Terminal,
    cursor_pos: Position,
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
            cursor_pos: Position{x: 0, y: 0},
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_postion(&Position {x: 0, y:0});
        if self.quit{
            Terminal::clear_screen();
            println!("Cya!\r");
        }
        else {
            self.draw_rows();
            Terminal::cursor_postion(&self.cursor_pos);
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
            Key::Up 
            | Key:: Down 
            | Key::Left 
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key:Key) {
        let Position {mut y, mut x} = self.cursor_pos;
        let size   = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width  = size.width.saturating_sub(1) as usize;
        match key{
            Key::Up    => y = y.saturating_sub(1),
            Key::Down  => {
                if y < height{
                    y = y.saturating_add(1);
                }
            },
            Key::Left  => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            Key::PageUp   => y = 0,
            Key::PageDown => y = height,
            Key::End      => x = 0,
            Key::Home     => x = width,
            _ => (),
        }
        self.cursor_pos = Position{x,y}
    }

    fn draw_welcome(&self) {
        let mut welcome = format!("Welcome to my text editor ");
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