use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor{
    quit:       bool,
    terminal:   Terminal,
    cursor_pos: Position,
    document:   Document,
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
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default() // No ; so that the document doesn't close
        };

        Self{
            quit:       false,
            terminal:   Terminal::default().expect("Failed to init terminal!"),
            cursor_pos: Position::default(),
            document,
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
            Terminal::cursor_postion(&Position::default());
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end   = self.terminal.size().width as usize;
        let row   = row.render(start,end);
        println!("{}\r",row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height - 1{
            Terminal::clear_ln();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if  self.document.is_empty() && terminal_row == height / 3{
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