pub mod curses;

use game::Game;

/// Indicates the direction of an arrow key.
pub enum ArrowType {
    Left, Right, Up, Down,
}

/// Indicates the type of a key.
pub enum KeyType {
    Backspace, Enter, Home, End,
}

/// Indicates the type of an input.
pub enum InputType {
    FirstFrame,
    Char(char),
    Arrow(ArrowType),
    Func(u8),
    Key(KeyType),
    Resize(u32, u32),
}

/// A game console.
pub trait Console {
    /// Returns a new Console.
    fn new() -> Self;

    /// Enters a rendering loop, consuming a Game.
    fn render<G>(&mut self, game: G) where G: Game;

    /// Gets width and height of the console, in tiles.
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}
