use crate::TryAdd;


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

        if x >= self.max_x {
            return None;
        }

        if y >= self.max_y {
            return None;
        }

        Some((x, y))
    }
}
