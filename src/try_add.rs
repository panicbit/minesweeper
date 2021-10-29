
pub trait TryAdd<Rhs = Self> {
    type Output;

    fn try_add(self, rhs: Rhs) -> Self::Output;
}

impl TryAdd for usize {
    type Output = Option<usize>;

    fn try_add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs)
    }
}

impl TryAdd<isize> for usize {
    type Output = Option<usize>;

    fn try_add(self, rhs: isize) -> Self::Output {
        let unsigned_rhs = rhs.unsigned_abs();

        if rhs.is_positive() {
            self.checked_add(unsigned_rhs)
        } else {
            self.checked_sub(unsigned_rhs)
        }
    }
}
