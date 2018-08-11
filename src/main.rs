#![feature(vec_resize_with)]

extern crate pancurses;
extern crate uuid;
extern crate rand;
extern crate num;

#[macro_use]
extern crate bitflags;

pub mod console;
pub mod game;
pub mod tile;
pub mod utils;
pub mod ui;

use console::Console;
use console::curses::CursesConsole;

use game::Game;
use game::endless::EndlessGame;

fn main() {
    let mut console = CursesConsole::new();
    let game = EndlessGame::new(console.get_width(), console.get_height());

    console.render(game);
}
