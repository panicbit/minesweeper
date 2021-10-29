#![allow(clippy::let_and_return)]

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

mod game;
pub use game::Game;

fn main() {
    let term = Term::new();
    let field = Field::with_mines(30, 10, 25).unwrap();
    let game = Game::new(term, field);

    game.run();
}
