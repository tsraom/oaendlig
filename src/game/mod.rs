pub mod endless;

use console::InputType;
use tile::Tile;
use ui::Ui;

pub trait Game {
    fn new(buf_width: usize, buf_height: usize) -> Self;
    fn react(&mut self, input: InputType) -> Vec<Ui>;
    fn gen_buffer(&mut self) -> Option<&Vec<Tile>>;
}
