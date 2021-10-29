use anyhow::*;
use rand::seq::IteratorRandom;

use crate::{Cell, TryAdd};

pub struct Field {
    cells: Vec<Cell>,
    width: usize,
}

impl Field {
    pub fn empty(width: usize, height: usize) -> Result<Self> {
        ensure!(width > 0, "field width must be greater than 0");
        ensure!(height > 0, "field height must be greater than 0");

        let size = width * height;
        let cells = vec![Cell::default(); size];

        Ok(Field {
            cells,
            width,
        })
    }

    pub fn with_mines(width: usize, height: usize, num_mines: usize) -> Result<Self> {
        let mut field = Self::empty(width, height)?;
        
        field.fill_with_mines(num_mines);

        Ok(field)
    }

    pub fn fill_with_mines(&mut self, num_mines: usize) {
        let rng = &mut rand::thread_rng();
        
        let selected_cells = self.cells.iter_mut()
            .filter(|cell| !cell.is_mine && !cell.is_revealed)
            .choose_multiple(rng, num_mines);

        for cell in selected_cells {
            cell.is_mine = true;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        let index = self.cell_index(x, y)?;

        Some(&self.cells[index])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let index = self.cell_index(x, y)?;

        Some(&mut self.cells[index])
    }

    /// Reveal cell.
    /// Returns `true` if a mine got hit.
    pub fn reveal(&mut self, x: usize, y: usize) -> bool {
        let cell = match self.get_mut(x, y) {
            Some(cell) => cell,
            None => return false,
        };

        if cell.is_revealed || cell.is_flagged {
            return false;
        }

        cell.is_revealed = true;

        if cell.is_mine {
            return true;
        }

        let num_neighbour_mines = self.num_neighbour_mines(x, y);

        // Cells without adjacent mines can recursively reveal their neighbours
        if num_neighbour_mines == 0 {
            for (x, y) in self.neighbour_positions(x, y) {
                self.reveal(x, y);
            }
        }

        false
    }

    pub fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = &Cell> {
        self.neighbour_positions(x, y)
            .filter_map(move |(x, y)| self.get(x, y))
    }

    pub fn num_neighbour_mines(&self, x: usize, y: usize) -> usize {
        self.neighbours(x, y)
            .filter(|cell| cell.is_mine)
            .count()
    }

    /// Calculates neighbour positions of `(x, y)`.
    /// Positions are not guaranteed to be inside the field.
    pub fn neighbour_positions(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let offsets = (-1..=1)
            .flat_map(|x: isize|
                (-1..=1).map(move |y: isize|
                    (x, y)
                )
            )
            .filter(|offset| *offset != (0, 0));

        let positions = offsets.filter_map(move |(x_off, y_off)| {
            let x = x.try_add(x_off)?;
            let y = y.try_add(y_off)?;

            Some((x, y))
        });

        positions
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

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Cell>> {
        self.cells
            .chunks(self.width())
            .map(|row| row.iter())
    }

    pub fn rows_enumerated(&self) -> impl Iterator<Item = (usize, impl Iterator<Item = (usize, &Cell)>)> {
        self.cells
            .chunks(self.width())
            .map(|row| row.iter().enumerate())
            .enumerate()
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.cells.len() / self.width()
    }

    pub fn size(&self) -> usize {
        self.cells.len()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_mut(x, y) {
            cell.is_flagged = !cell.is_flagged;
        }
    }
}
