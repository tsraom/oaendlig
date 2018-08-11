bitflags! {
    pub struct Style: u32 {
        const STANDOUT      = 1 << 0;
        const BOLD          = 1 << 1;
        const UNDERLINE     = 1 << 2;
        const DIM           = 1 << 3;
        
        const BLACK         = 1 << 4;
        const RED           = 1 << 5;
        const GREEN         = 1 << 6;
        const YELLOW        = 1 << 7;
        const BLUE          = 1 << 8;
        const MAGENTA       = 1 << 9;
        const CYAN          = 1 << 10;
        const WHITE         = 1 << 11;

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

