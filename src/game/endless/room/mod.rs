pub mod square;

use tile::Tile;

use uuid::Uuid;

pub trait Room {
    fn new(
        uuid: Uuid,
        num_doors: usize,
        width: usize,
        height: usize
    ) -> Self where Self: Sized;

    fn get_uuid(&self) -> Uuid;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile>;
}
