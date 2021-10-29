use crate::TryAdd;

#[derive(Debug)]
pub struct Cursor {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
}

impl Cursor {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        Self {
            x: 0,
            y: 0,
            max_x,
            max_y,
        }
    }

    pub fn is_at(&self, x: usize, y: usize) -> bool {
        self.x == x && self.y == y
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn set_x(&mut self, x: usize) {
        self.set_position(x, self.y())
    }

    pub fn set_y(&mut self, y: usize) {
        self.set_position(self.x(), y)
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        if !self.position_is_valid(x, y) {
            return;
        }
        
        self.x = x;
        self.y = y;
    }

    pub fn position_is_valid(&self, x: usize, y: usize) -> bool {
        x < self.max_x && y < self.max_y
    }

    pub fn up(&mut self) {
        self.apply_offset(0, -1);
    }

    pub fn down(&mut self) {
        self.apply_offset(0, 1);
    }

    pub fn left(&mut self) {
        self.apply_offset(-1, 0);
    }

    pub fn right(&mut self) {
        self.apply_offset(1, 0);
    }

    pub fn apply_offset(&mut self, x_off: isize, y_off: isize) {
        if let Some((x, y)) = self.offset_position(x_off, y_off) {
            self.x = x;
            self.y = y;
        }
    }

    fn offset_position(&self, x_off: isize, y_off: isize) -> Option<(usize, usize)> {
        let x = self.x.try_add(x_off)?;
        let y = self.y.try_add(y_off)?;

        if !self.position_is_valid(x, y) {
            return None;
        }

        Some((x, y))
    }
}
