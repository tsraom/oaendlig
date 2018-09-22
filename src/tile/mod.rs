pub mod base;
pub mod cover;
pub mod style;

use self::base::Base;
use self::cover::Cover;
use self::style::Style;

use std::mem;

#[derive(Clone)]
pub struct Tile {
    pub base: Base,
    pub cover: Option<Cover>,
    pub style: Style,
}

impl Tile {
    pub fn new(base: Base, cover: Option<Cover>, style: Style) -> Tile {
        Tile {
            base: base,
            cover: cover,
            style: style,
        }
    }

    pub fn ground() -> Tile {
        Tile {
            base: Base::Ground,
            cover: Option::default(),
            style: Style::DEF_DIM,
        }
    }

    pub fn wall() -> Tile {
        Tile {
            base: Base::Wall,
            cover: Option::default(),
            style: Style::default(),
        }
    }

    pub fn door(index: usize) -> Tile {
        Tile {
            base: Base::Door(index),
            cover: Option::default(),
            style: Style::DEF_BOLD,
        }
    }

    pub fn void() -> Tile {
        Tile {
            base: Base::Void,
            cover: Option::default(),
            style: Style::default(),
        }
    }

    pub fn player() -> Tile {
        Tile {
            base: Base::Player,
            cover: Option::default(),
            style: Style::default(),
        }
    }

    pub fn has_base(&self, base: &Base) -> bool {
        mem::discriminant(&self.base) == mem::discriminant(base)
    }
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            base: Base::default(),
            cover: Option::default(),
            style: Style::default(),
        }
    }
}
