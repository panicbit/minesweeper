#![allow(unused)]

use anyhow::*;
use crossterm::{event::{Event, KeyCode, KeyEvent, KeyModifiers}, style::{Attribute, Color, ContentStyle, StyledContent, Stylize}};
use rand::seq::IteratorRandom;

mod field;
pub use field::Field;

mod cell;
pub use cell::Cell;

mod try_add;
pub use try_add::TryAdd;

mod cursor;
pub use cursor::Cursor;

mod term;
pub use term::Term;

fn main() {
    let term = Term::new();
    let field = Field::with_mines(30, 10, 5).unwrap();
    let game = Game::new(term, field);

    game.run();
}

struct Game {
    term: Term,
    field: Field,
    cursor: Cursor,
}

impl Game {
    fn new(term: Term, field: Field) -> Self {
        let mut cursor = Cursor::new(field.width(), field.height());

        Self {
            term,
            field,
            cursor,
        }
    }

    fn render(&self, show_revealed: bool) {
        self.term.clear();

        for (y, row) in self.field.rows_enumerated() {
            for (x, cell) in row {
                self.render_cell(cell, x, y, show_revealed);
            }

            println!();
        }
    }

    fn render_cell(&self, cell: &Cell, x: usize, y: usize, show_revealed: bool) {
        let mut style = ContentStyle::new();

        let is_highlighted = self.cursor.is_at(x, y);

        if is_highlighted {
            style.attributes.set(Attribute::Reverse);
        }

        if !cell.is_revealed {
            self.term.print(style.apply("#"));
            return;
        }

        if cell.is_mine {
            self.term.print(style.apply("*").red());
            return;
        }
        
        let num_neighbour_mines = self.field.num_neighbour_mines(x, y);

        let style = match num_neighbour_mines {
            0 => style,
            1 => style.green(),
            2 => style.red(),
            3 => style.yellow(),
            4 => style.blue(),
            _ => style.magenta(),
        };

        if num_neighbour_mines == 0 {
            self.term.print(style.apply(" "));
        } else {
            self.term.print(style.apply(num_neighbour_mines));
        }
    }

    fn reveal_at_cursor(&mut self) {
        let (x, y) = self.cursor.position();

        self.field.reveal(x, y);
    }

    fn read_input(&self) -> Action {
        match self.term.read_input() {
            Event::Key(key) if key.modifiers == KeyModifiers::CONTROL => match key.code {
                KeyCode::Char('c') => Action::Quit,
                _ => Action::Redraw,
            },
            Event::Key(key) => match key.code {
                KeyCode::Up => Action::Up,
                KeyCode::Down => Action::Down,
                KeyCode::Left => Action::Left,
                KeyCode::Right => Action::Right,
                KeyCode::Enter => Action::RevealAtCursor,
                KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
                _ => Action::Redraw,
            },
            Event::Mouse(_) => Action::Redraw,
            Event::Resize(_, _) => Action::Redraw,
        }
    }

    fn run(mut self) {
        loop {
            let show_revealed = false;

            self.render(true);

            match self.outcome() {
                Outcome::Pending => {},
                Outcome::Won => {
                    println!("YOU WON!");
                    return;
                },
                Outcome::Lost => {
                    println!("YOU LOST!");
                    return;
                }
            }

            match self.read_input() {
                Action::Quit => return,
                Action::RevealAtCursor => self.reveal_at_cursor(),
                Action::Up => self.cursor.up(),
                Action::Down => self.cursor.down(),
                Action::Left => self.cursor.left(),
                Action::Right => self.cursor.right(),
                Action::Redraw => continue,
            }
        }
    }

    fn outcome(&self) -> Outcome {
        let mut all_cells_revealed = true;

        for cell in self.field.cells() {
            if !cell.is_revealed && !cell.is_mine  {
                all_cells_revealed = false;
            }

            if cell.is_revealed && cell.is_mine  {
                return Outcome::Lost;
            }
        }

        match all_cells_revealed {
            true => Outcome::Won,
            false => Outcome::Pending,
        }
    }
}

enum Action {
    Quit,
    RevealAtCursor,
    Up,
    Down,
    Left,
    Right,
    Redraw,
}

enum Outcome {
    Pending,
    Won,
    Lost,
}
