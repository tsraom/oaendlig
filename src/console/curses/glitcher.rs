//  stupid Self not matching enum issue addressed in #26264, #31168

use utils::{ rnd_lt, rnd_ch, clamp };

use pancurses::*;

enum GlitchPosition {
    Keep,
    ShiftX(isize),
    ShiftY(isize),
    ShiftXY(isize, isize),
}

impl GlitchPosition {
    fn rnd() -> Self {
        match rnd_lt::<u8>(4) {
            0 => GlitchPosition::Keep,
            1 => GlitchPosition::ShiftX(rnd_lt::<isize>(10) - 5),
            2 => GlitchPosition::ShiftY(rnd_lt::<isize>(10) - 5),
            _ => GlitchPosition::ShiftXY(
                rnd_lt::<isize>(10) - 5,
                rnd_lt::<isize>(10) - 5
                ),
        }
    }

    fn get_ch(&self, x: usize, y: usize, w: usize, h: usize, buffer: &Vec<chtype>) -> chtype {
        match self {
            GlitchPosition::Keep => A_NORMAL,
            GlitchPosition::ShiftX(sx) => {
                let ax = clamp(x as isize + sx + (y * 5 / h) as isize, 0, w as isize - 1) as usize;
                buffer[y * w + ax]
            },

            GlitchPosition::ShiftY(sy) => {
                let ay = clamp(y as isize + sy + (x * 5 / w) as isize, 0, h as isize - 1) as usize;
                buffer[ay * w + x]
            },

            GlitchPosition::ShiftXY(sx, sy) => {
                let ax = clamp(x as isize + sx, 0, w as isize - 1) as usize;
                let ay = clamp(y as isize + sy, 0, h as isize - 1) as usize;
                buffer[ay * w + ax]
            },
        }
    }
}

enum GlitchChar {
    Keep,
    Char(chtype),
    Shift(usize),
    Endless,
}

impl GlitchChar {
    fn rnd() -> Self {
        match rnd_lt::<u8>(4) {
            0 => GlitchChar::Keep,
            1 => GlitchChar::Char(rnd_ch() as chtype),
            2 => GlitchChar::Shift(rnd_lt::<usize>(10)),
            _ => GlitchChar::Endless,
        }
    }

    fn get_ch(&self, x: usize, y: usize, ch: chtype) -> chtype {
        match self {
            GlitchChar::Keep => A_NORMAL,
            GlitchChar::Char(the_ch) => *the_ch,
            GlitchChar::Shift(x) => match char::is_ascii_graphic(&((ch + *x as chtype) as u8 as char)) {
                true => ch + *x as chtype,
                false => ch,
            },

            GlitchChar::Endless => (match (x + y) % 8 {
                0 => 'o',
                1 => 'ä',
                2 => 'n',
                3 => 'd',
                4 => 'l',
                5 => 'i',
                6 => 'g',
                _ => ' ',
            }) as chtype,
        }
    }
}

enum GlitchStyle {
    Keep,
    Style(chtype),
}

impl GlitchStyle {
    fn rnd() -> Self {
        match rnd_lt::<u8>(2) {
            0 => GlitchStyle::Keep,
            _ => GlitchStyle::Style(match rnd_lt::<u8>(6) {
                0 => A_ITALIC,
                1 => A_REVERSE,
                2 => A_BOLD,
                3 => A_UNDERLINE,
                4 => A_STRIKEOUT,
                _ => A_LEFTLINE,
            } | COLOR_PAIR(match rnd_lt::<u8>(4) {
                0 => 0,
                1 => 1,
                2 => 6,
                _ => 3,
            })),
        }
    }

    fn get_ch(&self) -> chtype {
        match self {
            GlitchStyle::Keep => A_NORMAL,
            GlitchStyle::Style(s) => *s,
        }
    }
}

struct GlitchType {
    pos: GlitchPosition,
    ch: GlitchChar,
    style: GlitchStyle,
}

impl GlitchType {
    fn rnd() -> Self {
        GlitchType {
            pos: GlitchPosition::rnd(),
            ch: GlitchChar::rnd(),
            style: GlitchStyle::rnd()
        }
    }

    fn get_ch(&self, ch: chtype, x: usize, y: usize, w: usize, h: usize, buffer: &Vec<chtype>) -> chtype {
        let ch = match self.pos {
            GlitchPosition::Keep => self.ch.get_ch(x, y, ch),
            _ => self.pos.get_ch(x, y, w, h, buffer),
        };

        ch | self.style.get_ch()
    }
}

struct GlitchRect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub ty: GlitchType,
}

impl GlitchRect {
    fn new(ww: usize, wh: usize) -> Self {
        let x = rnd_lt::<usize>(ww);
        let y = rnd_lt::<usize>(wh);

        Self {
            x: x,
            y: y,
            w: rnd_lt(ww - x),
            h: rnd_lt(wh - y),
            ty: GlitchType::rnd(),
        }
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }

    fn get_ch(&self, ch: chtype, x: usize, y: usize, w: usize, h: usize, buffer: &Vec<chtype>) -> chtype {
        self.ty.get_ch(ch, x, y, w, h, buffer)
    }
}

pub struct Glitcher {
    buffer: Vec<chtype>,
    rects: Vec<GlitchRect>,
    width: usize,
    height: usize,
}

impl Glitcher {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            rects: Vec::new(),
            width: width,
            height: height,
        }
    }

    pub fn resize(&mut self, ww: usize, wh: usize) {
        self.width = ww;
        self.height = wh;
        self.buffer.resize(ww * wh, 0);
    }

    pub fn write(&mut self, x: usize, y: usize, ch: chtype) {
        let index = y * self.width + x;
        if index < self.buffer.len() {
            self.buffer[index] = Self::glitch_ch(ch, 1000);
        }
    }

    pub fn write_str(&mut self, x: usize, y: usize, s: &str) {
        //  don't write if str causes overflow
        let slen = s.len();
        if slen <= self.width - x {
            let index = y * self.width + x;
            if index < self.buffer.len() {
                for (i, ch) in s.chars().enumerate() {
                    self.buffer[index + i] = Self::glitch_ch(
                        ch as chtype, 1000
                    );
                }
            }
        }
    }

    pub fn update(&mut self) {
        match rnd_lt::<usize>(5) {
            0...1 => {
                if self.rects.len() < 10 {
                    self.rects.push(GlitchRect::new(self.width, self.height));
                }
            },

            2 => {
                if !self.rects.is_empty() {
                    let len = self.rects.len();
                    self.rects.remove(rnd_lt(len));
                }
            },

            3...4 => {
                if self.rects.is_empty() {
                    return;
                }

                let index = rnd_lt(self.rects.len());
                match rnd_lt::<u8>(4) {
                    0 => {
                        if
                            self.rects[index].w > 1 &&
                            self.rects[index].x < self.rects[index].w - 1
                        {
                            self.rects[index].x += 1;
                            self.rects[index].w -= 1;
                        }
                    },

                    1 => {
                        if self.rects[index].x > 0 {
                            self.rects[index].x -= 1;
                            self.rects[index].w += 1;
                        }
                    },

                    2 => {
                        if
                            self.rects[index].h > 1 &&
                            self.rects[index].y < self.rects[index].h - 1
                        {
                            self.rects[index].y += 1;
                            self.rects[index].y -= 1;
                        }
                    },

                    3 => {
                        if self.rects[index].y > 1 {
                            self.rects[index].y -= 1;
                            self.rects[index].h += 1;
                        }
                    },

                    _ => (),
                }
            },

            _ => (),
        }
    }

    pub fn render(&self, window: &Window) {
        let mut x: usize = 0;
        let mut y: usize = 0;

        window.clear();
        window.mv(0, 0);
        for ch in self.buffer.iter() {
            window.addch(self.render_ch(*ch, x, y));
            x += 1;
            x %= self.width;
            if (x == 0) {
                y += 1;
            }
        }
    }

    fn render_ch(&self, ch: chtype, x: usize, y: usize) -> chtype {
        for rect in self.rects.iter() {
            if rect.contains(x, y) {
                return ch | rect.get_ch(ch, x, y, self.width, self.height, &self.buffer);
            }
        }

        ch
    }

    fn glitch_ch(ch: chtype, chance: usize) -> chtype {
        match chance > 0 && rnd_lt::<usize>(chance) == 0 {
            true => Self::rnd_ch(),
            false => ch,
        }
    }

    fn rnd_ch() -> chtype {
        match rnd_lt::<u8>(17) {
            0 => ACS_ULCORNER(),
            1 => ACS_URCORNER(),
            2 => ACS_LLCORNER(),
            3 => ACS_LRCORNER(),
            4 => ACS_LTEE(),
            5 => ACS_RTEE(),
            6 => ACS_BTEE(),
            7 => ACS_TTEE(),
            8 => ACS_HLINE(),
            9 => ACS_VLINE(),
            10 => ACS_S1(),
            11 => ACS_S3(),
            12 => ACS_S7(),
            13 => ACS_S9(),
            14 => ACS_DIAMOND(),
            15 => ACS_DEGREE(),
            16 => ACS_BULLET(),
            _ => ' ' as chtype,
        }
    }
}
