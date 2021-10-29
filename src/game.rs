use crossterm::event::{Event, KeyCode, KeyModifiers, MouseButton, MouseEventKind};
use crossterm::style::{Attribute, ContentStyle, Stylize};

use crate::{Cell, Cursor, Field, Term};

pub struct Game {
    term: Term,
    field: Field,
    cursor: Cursor,
    held_mouse_button: Option<MouseButton>,
}

impl Game {
    pub fn new(term: Term, field: Field) -> Self {
        let cursor = Cursor::new(field.width(), field.height());

        Self {
            term,
            field,
            cursor,
            held_mouse_button: None,
        }
    }

    pub fn run(mut self) {
        loop {
            self.render();

            match self.outcome() {
                Outcome::Pending => {},
                Outcome::Won => {
                    self.term.println("YOU WON!");
                    return;
                },
                Outcome::Lost => {
                    self.term.println("YOU LOST!");
                    return;
                }
            }

            match self.read_input() {
                Action::Quit => return,
                Action::RevealAt(x, y) => self.reveal_at(x, y),
                Action::RevealAtCursor => self.reveal_at_cursor(),
                Action::ToggleFlagAt(x, y) => self.toggle_flag_at(x, y),
                Action::ToggleFlagAtCursor => self.toggle_flag_at_cursor(),
                Action::Up => self.cursor.up(),
                Action::Down => self.cursor.down(),
                Action::Left => self.cursor.left(),
                Action::Right => self.cursor.right(),
                Action::Redraw => continue,
            }
        }
    }

    fn render(&self) {
        self.term.clear();

        for (y, row) in self.field.rows_enumerated() {
            for (x, cell) in row {
                self.render_cell(cell, x, y);
            }

            self.term.println("");
        }
    }

    fn render_cell(&self, cell: &Cell, x: usize, y: usize) {
        let mut style = ContentStyle::new();

        let is_highlighted = self.cursor.is_at(x, y);

        if is_highlighted {
            style.attributes.set(Attribute::Reverse);
        }

        if !cell.is_revealed {
            if cell.is_flagged {
                self.term.print(style.apply("!").on_red());
            } else {
                self.term.print(style.apply("#"));
            }
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

    fn reveal_at(&mut self, x: usize, y: usize) {
        self.cursor.set_position(x, y);
        self.field.reveal(x, y);
    }

    fn toggle_flag_at_cursor(&mut self) {
        let (x, y) = self.cursor.position();

        self.field.toggle_flag(x, y);
    }

    fn toggle_flag_at(&mut self, x: usize, y: usize) {
        self.cursor.set_position(x, y);
        self.field.toggle_flag(x, y);
    }

    fn read_input(&mut self) -> Action {
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
                KeyCode::Char(' ') => Action::ToggleFlagAtCursor,
                KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
                _ => Action::Redraw,
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::Down(button) => {
                    self.held_mouse_button = Some(button);
                    self.read_input()
                }
                MouseEventKind::Up(_) => match self.held_mouse_button.take() {
                    Some(MouseButton::Left) => {
                        let x = mouse.column as usize;
                        let y = mouse.row as usize;
                        
                        Action::RevealAt(x, y)
                    },
                    Some(MouseButton::Right) => {
                        let x = mouse.column as usize;
                        let y = mouse.row as usize;

                        Action::ToggleFlagAt(x, y)
                    }
                    _ => self.read_input(),
                },
                _ => self.read_input(),
            },
            Event::Resize(_, _) => Action::Redraw,
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
    RevealAt(usize, usize),
    RevealAtCursor,
    ToggleFlagAt(usize, usize),
    ToggleFlagAtCursor,
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
