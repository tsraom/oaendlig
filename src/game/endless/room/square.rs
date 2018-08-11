use tile::Tile;
use game::endless::room::Room;

use std::cmp;
use uuid::Uuid;
use rand::prelude::*;

pub struct SquareRoom {
    uuid: Uuid,
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl SquareRoom {
    fn fill_tiles(&mut self, width: usize, height: usize, num_doors: usize) {
        let size = width * height;

        let mut num_doors_left = num_doors;
        let mut num_walls_left = width * 2 + height * 2 - 8;

        for y in 0..height {
            for x in 0..width {
                let is_wall = (x == 0) || (x == width - 1) || (y == 0) || (y == height - 1);

                let min_clamp = cmp::min(
                    size / 2,
                    num_walls_left - num_doors_left
                );
                let valid_door = self.is_valid_door_position(x, y);
                let is_door =
                    valid_door &&
                    num_doors_left > 0 &&
                    random::<usize>() % min_clamp == 0;

                match is_wall {
                    false => { self.tiles.push(Tile::default()); },
                    true => {
                        match is_door {
                            false => { self.tiles.push(Tile::wall()); },
                            true => {
                                self.tiles.push(Tile::door(num_doors - num_doors_left));

                                if num_doors_left > 0 {
                                    num_doors_left -= 1;
                                }
                            },
                        };

                        if valid_door && num_walls_left > 0 {
                            num_walls_left -= 1;
                        }
                    },
                }
            }
        }

        if num_doors_left > 0 {
            panic!("need doors");
        };
    }

    fn is_valid_door_position(&self, x: usize, y: usize) -> bool {
        //  can't place door in corner
        if (x == 0 || x == self.width - 1) && (y == 0 || y == self.height - 1) {
            return false;
        }

        true
        /*
        //  can't place door next to another door
        match self.get_tile(x - 1, y)
            .or_else(|| self.get_tile(x + 1, y))
            .or_else(|| self.get_tile(x, y - 1))
            .or_else(|| self.get_tile(x, y + 1))
        {
            Some(ref tile) => tile.has_base(&Base::Door(42)),
            None => true,
        }*/
    }
}

impl Room for SquareRoom {
    fn new(uuid: Uuid, num_doors: usize, width: usize, height: usize) -> Self {
        let mut res = Self {
            uuid: uuid,
            width: width,
            height: height,
            tiles: Vec::new(),
        };

        res.fill_tiles(width, height, num_doors);
        res
    }

    fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        match x >= self.width || y >= self.height {
            true => None,
            false => Some(&self.tiles[y * self.width + x]),
        }
    }
}
