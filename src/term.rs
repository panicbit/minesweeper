use std::cell::RefCell;
use std::io::{self, Stdout, Write};

use crossterm::style::{ContentStyle, Print};
use crossterm::{QueueableCommand, event};
use crossterm::cursor::{Hide, MoveTo, MoveToRow, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{self, DisableLineWrap, EnableLineWrap};

use crate::Cursor;

pub struct Term {
    stdout: RefCell<Stdout>,
    back_buffer: RefCell<Buffer>,
}

impl Term {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Term {
        let (width, height) = crossterm::terminal::size().unwrap();
        let width = width as usize;
        let height = height as usize;

        let mut term = Self {
            stdout: RefCell::new(io::stdout()),
            back_buffer: RefCell::new(Buffer::new(width, height)),
        };

        term.term_init();

        term
    }

    pub fn print_char(&self, content: char, style: ContentStyle) {
        self.back_buffer.borrow_mut().put(Cell::new(content, style));
    }

    pub fn print(&self, text: &str, style: ContentStyle) {
        let mut back_buffer = self.back_buffer.borrow_mut();

        for char in text.chars() {
            back_buffer.put(Cell::new(char, style));
        }
    }

    pub fn println(&self, text: &str, style: ContentStyle) {
        self.print(text, style);
        self.new_line();
    }

    pub fn new_line(&self) {
        self.back_buffer.borrow_mut().new_line();
    }

    pub fn clear(&self) {
        self.back_buffer.borrow_mut().clear();
    }

    pub fn read_input(&self) -> Event {
        let event = event::read().unwrap();

        if let Event::Resize(x, y) = event {
            let x = x as usize;
            let y = y as usize;

            self.back_buffer.borrow_mut().resize(x, y);
        }

        event
    }

    pub fn present(&self) {
        let mut back_buffer = self.back_buffer.borrow_mut();
        let mut stdout = self.stdout.borrow_mut();

        for (y, row) in back_buffer.rows_enumerated_mut() {
            for (x, cell) in row {
                if !cell.dirty {
                    continue;
                }

                let x = x as u16;
                let y = y as u16;

                stdout.queue(MoveTo(x, y)).ok();
                stdout.queue(Print(cell.style.apply(cell.content))).ok();
            }
        }

        stdout.flush().ok();
    }

    fn truncate_empty_space(&self) {
        let mut back_buffer = self.back_buffer.borrow_mut();


        for (y, mut row) in back_buffer.rows_enumerated_mut().rev() {
            if row.any(|(_, cell)| !cell.is_clear()) {
                let y = y as u16;

                self.stdout.borrow_mut().queue(MoveToRow(y + 1)).ok();

                return;
            }
        }
    }

    fn term_init(&mut self) {
        let mut stdout = self.stdout.borrow_mut();

        terminal::enable_raw_mode().ok();

        stdout.queue(Hide).ok();
        stdout.queue(EnableMouseCapture).ok();
        stdout.queue(DisableLineWrap).ok();
        stdout.flush().ok();
    }

    fn term_shutdown(&mut self) {
        let mut stdout = self.stdout.borrow_mut();

        stdout.queue(EnableLineWrap).ok();
        stdout.queue(DisableMouseCapture).ok();
        stdout.queue(Show).ok();
        stdout.flush().ok();

        terminal::disable_raw_mode().ok();
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        self.truncate_empty_space();
        self.term_shutdown();
    }
}

struct Buffer {
    cells: Vec<Cell>,
    width: usize,
    cursor: Cursor,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        let size = width * height;

        // Allow the cursor to run outside of the buffer
        let cursor = Cursor::new(width + 1, height + 1);

        Self {
            cells: vec![Cell::new(' ', ContentStyle::new()); size],
            width,
            cursor,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        *self = Self::new(width, height);
    }

    pub fn clear(&mut self) {
        let clear_cell = Cell::new(' ', ContentStyle::new());

        self.cursor.set_position(0, 0);

        for cell in &mut self.cells {
            cell.put(clear_cell);
        }
    }

    pub fn new_line(&mut self) {
        self.cursor.set_x(0);
        self.cursor.down();
    }

    pub fn put(&mut self, new_cell: Cell) {
        if new_cell.content == '\n' {
            self.new_line();
            return;
        }

        if let Some(cell) = self.get_current_mut() {
            cell.put(new_cell);
            self.cursor.right();
        }
    }

    pub fn get_current_mut(&mut self) -> Option<&mut Cell> {
        let (x, y) = self.cursor.position();

        self.get_mut(x, y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let index = self.cell_index(x, y)?;

        Some(&mut self.cells[index])
    }

    fn cell_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width() {
            return None;
        }

        let index = y * self.width() + x;

        if index >= self.cells.len() {
            return None;
        }

        Some(index)
    }

    pub fn rows_enumerated_mut(&mut self) -> impl DoubleEndedIterator<Item = (usize, impl DoubleEndedIterator<Item = (usize, &mut Cell)>)> {
        let width = self.width();

        self.cells
            .chunks_mut(width)
            .map(|row| row.iter_mut().enumerate())
            .enumerate()
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

#[derive(Copy, Clone)]
struct Cell {
    content: char,
    style: ContentStyle,
    dirty: bool,
}

impl Cell {
    fn new(content: char, style: ContentStyle) -> Self {
        Self {
            content,
            style,
            dirty: true,
        }
    }

    fn put(&mut self, other: Cell) {
        if self.content == other.content && self.style == other.style {
            return
        }

        *self = other;
    }

    fn is_clear(&self) -> bool {
        self.content == ' ' && self.style == ContentStyle::new()
    }
}
