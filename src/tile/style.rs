use utils::rnd_within;

bitflags! {
    pub struct Style: u32 {
        const STANDOUT      = 1 << 0;
        const BOLD          = 1 << 1;
        const UNDERLINE     = 1 << 2;
        const DIM           = 1 << 3;
        
        const DARK1        = 1 << 4;
        const DARK2        = 1 << 5;
        const DARK3        = 1 << 6;
        const DARK4        = 1 << 7;
        const DARK5        = 1 << 8;
        const DARK6        = 1 << 9;
        const DARK7        = 1 << 10;
        const DARK8        = 1 << 11;
       
        const BLACK         = 1 << 12;
        const RED           = 1 << 13;
        const GREEN         = 1 << 14;
        const YELLOW        = 1 << 15;
        const BLUE          = 1 << 16;
        const MAGENTA       = 1 << 17;
        const CYAN          = 1 << 18;
        const WHITE         = 1 << 19;

        const DEF           = Self::BLACK.bits;
        const DEF_STANDOUT  = Self::BLACK.bits | Self::STANDOUT.bits;
        const DEF_BOLD      = Self::BLACK.bits | Self::BOLD.bits;
        const DEF_UNDERLINE = Self::BLACK.bits | Self::UNDERLINE.bits;
        const DEF_DIM       = Self::BLACK.bits | Self::DIM.bits;
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::DEF
    }
}

impl Style {
    pub fn rnd_color() -> Self {
        //  1 << rnd_within::<u32>(4..20))
        match rnd_within::<u8>(4..20) {
            4 => Style::DARK1,
            5 => Style::DARK2,
            6 => Style::DARK3,
            7 => Style::DARK4,
            8 => Style::DARK5,
            9 => Style::DARK6,
            10 => Style::DARK7,
            11 => Style::DARK8,
            12 => Style::BLACK,
            13 => Style::RED,
            14 => Style::GREEN,
            15 => Style::YELLOW,
            16 => Style::BLUE,
            17 => Style::MAGENTA,
            18 => Style::CYAN,
            19 => Style::WHITE,
            _ => panic!(),
        }
    }
}
