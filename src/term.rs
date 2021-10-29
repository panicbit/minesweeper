use std::cell::RefCell;
use std::fmt::Display;
use std::io::{self, Stdout};

use crossterm::style::Print;
use crossterm::{ExecutableCommand, event};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{self, Clear, ClearType};

pub struct Term {
    stdout: RefCell<Stdout>,
}

impl Term {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Term {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode().ok();

        stdout.execute(Hide).ok();
        stdout.execute(EnableMouseCapture).ok();

        Self {
            stdout: RefCell::new(stdout),
        }
    }

    pub fn print(&self, text: impl Display) {
        self.stdout.borrow_mut().execute(Print(text)).ok();
    }

    pub fn clear(&self) {
        self.stdout.borrow_mut().execute(Clear(ClearType::All)).ok();
        self.move_to(0, 0);
    }

    pub fn move_to(&self, x: u16, y: u16) {
        self.stdout.borrow_mut().execute(MoveTo(x, y)).ok();
    }

    pub fn read_input(&self) -> Event {
        event::read().unwrap()
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        self.stdout.borrow_mut().execute(DisableMouseCapture).ok();
        self.stdout.borrow_mut().execute(Show).ok();
        terminal::disable_raw_mode().ok();
    }
}
