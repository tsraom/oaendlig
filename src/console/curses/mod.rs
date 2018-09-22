pub mod messages;
mod glitcher;

use console::{ Console, InputType, ArrowType, KeyType };
use console::curses::messages::bottom::BottomMessage;
use console::curses::glitcher::Glitcher;
use game::Game;
use tile::Tile;
use tile::base::Base;
use tile::style::Style;
use ui::Ui;
use utils::rnd_lt;

use pancurses::*;
use rand::prelude::*;

use std::{thread, time};

const CLR_BLACK: i16 = 0;
const CLR_RED: i16 = 1;
const CLR_GREEN: i16 = 2;
const CLR_YELLOW: i16 = 3;
const CLR_BLUE: i16 = 4;
const CLR_MAGENTA: i16 = 5;
const CLR_CYAN: i16 = 6;
const CLR_WHITE: i16 = 7;

const CLR_DARK1: i16 = 8;
const CLR_DARK2: i16 = 9;
const CLR_DARK3: i16 = 10;
const CLR_DARK4: i16 = 11;
const CLR_DARK5: i16 = 12;
const CLR_DARK6: i16 = 13;
const CLR_DARK7: i16 = 14;
const CLR_DARK8: i16 = 15;

pub struct CursesConsole {
    window: Window,
    message: Option<BottomMessage>,
    frame: usize,
    glitcher: Glitcher,
    use_colors: bool,
}

impl CursesConsole {
    fn make_style(&self, style: &Style) -> chtype {
        let mut res = A_NORMAL;

        if style.intersects(Style::STANDOUT) {
            //  for some reason A_STANDOUT doesn't compile
            //  res |= A_STANDOUT;
        }

        if style.intersects(Style::BOLD) {
            res |= A_BOLD;
        }

        if style.intersects(Style::UNDERLINE) {
            res |= A_UNDERLINE;
        }

        if style.intersects(Style::DIM) {
            res |= A_DIM;
        }

        if self.use_colors {
            if style.intersects(Style::BLACK) {
                res |= COLOR_PAIR(CLR_BLACK as u64);
            }

            if style.intersects(Style::RED) {
                res |= COLOR_PAIR(CLR_RED as u64);
            }

            if style.intersects(Style::GREEN) {
                res |= COLOR_PAIR(CLR_GREEN as u64);
            }

            if style.intersects(Style::YELLOW) {
                res |= COLOR_PAIR(CLR_YELLOW as u64);
            }

            if style.intersects(Style::BLUE) {
                res |= COLOR_PAIR(CLR_BLUE as u64);
            }

            if style.intersects(Style::MAGENTA) {
                res |= COLOR_PAIR(CLR_MAGENTA as u64);
            }

            if style.intersects(Style::CYAN) {
                res |= COLOR_PAIR(CLR_CYAN as u64);
            }

            if style.intersects(Style::WHITE) {
                res |= COLOR_PAIR(CLR_WHITE as u64);
            }

            if style.intersects(Style::DARK1) {
                res |= COLOR_PAIR(CLR_DARK1 as u64);
            }

            if style.intersects(Style::DARK2) {
                res |= COLOR_PAIR(CLR_DARK2 as u64);
            }

            if style.intersects(Style::DARK3) {
                res |= COLOR_PAIR(CLR_DARK3 as u64);
            }

            if style.intersects(Style::DARK4) {
                res |= COLOR_PAIR(CLR_DARK4 as u64);
            }

            if style.intersects(Style::DARK5) {
                res |= COLOR_PAIR(CLR_DARK5 as u64);
            }

            if style.intersects(Style::DARK6) {
                res |= COLOR_PAIR(CLR_DARK6 as u64);
            }

            if style.intersects(Style::DARK7) {
                res |= COLOR_PAIR(CLR_DARK7 as u64);
            }

            if style.intersects(Style::DARK8) {
                res |= COLOR_PAIR(CLR_DARK8 as u64);
            }
        }

        res
    }

    fn draw(&mut self, buffer: &Vec<Tile>) {
        let mut x: usize = 0;
        let mut y: usize = 0;
        let ww: usize = self.get_width();

        for tile in buffer.iter() {
            let base = &tile.base;
            let cover = &tile.cover;
            let style = &tile.style;

            let ch = match base {
                &Base::Void => ' ',
                &Base::Player => 'o',
                &Base::Ground => '.',
                &Base::Wall => 'X',
                &Base::Water => '=',
                &Base::Message(ref s) => '?',
                &Base::Door(ref x) => 'd',
            };

            let ch = (ch as chtype) | self.make_style(style);
            self.glitcher.write(x, y, ch);

            x += 1;
            x %= ww;
            if (x == 0) {
                y += 1;
            }
        }
    }

    fn draw_message(&mut self) {
        if let None = self.message {
            return;
        }

        let msg = self.message.as_ref().unwrap();

        //  margins
        let xm: usize = 1;
        let ym: usize = 0;

        let ww = self.get_width();
        let wh = self.get_height();

        let text = msg.get_text();
        let text_width = ww - 2 - 2 * xm;
        let mut num_lines = text.len() / text_width;
        if text.len() % text_width > 0 {
            num_lines += 1;
        }

        //  if we don't have enough space to draw the message, don't draw
        if wh < 2 + 2 * ym + num_lines {
            return;
        }

        let sx = 0;
        let sy = wh - 2 - 2 * ym - num_lines;

        for y in sy..wh {
            for x in sx..ww {
                let ch = match (x == sx, x == ww - 1, y == sy, y == wh - 1) {
                    (true, _, true, _) => ACS_ULCORNER(),
                    (true, _, _, true) => ACS_LLCORNER(),
                    (_, true, true, _) => ACS_URCORNER(),
                    (_, true, _, true) => ACS_LRCORNER(),
                    (_, _, true, _) => ACS_HLINE(),
                    (_, _, _, true) => ACS_HLINE(),
                    (true, _, _, _) => ACS_VLINE(),
                    (_, true, _, _) => ACS_VLINE(),
                    _ => ' ' as chtype,
                };

                self.glitcher.write(x, y, ch);
            }
        }

        for line in 0..num_lines {
            let start = line * text_width;
            let end = match line == num_lines - 1 {
                true => start + text.len() % text_width,
                false => (line + 1) * text_width,
            };

            self.glitcher.write_str(
                sx + 1 + xm,
                sy + 1 + ym + line,
                &text[start..end]
            );
        }
    }



    fn init_colors(&mut self) {
        if
            has_colors() &&
            can_change_color() &&
            COLORS() >= 16 &&
            COLOR_PAIRS() >= 16
        {
            self.use_colors = true;

            init_color(CLR_DARK1, 111, 111, 111);
            init_color(CLR_DARK2, 222, 222, 222);
            init_color(CLR_DARK3, 333, 333, 333);
            init_color(CLR_DARK4, 444, 444, 444);
            init_color(CLR_DARK5, 555, 555, 555);
            init_color(CLR_DARK6, 666, 666, 666);
            init_color(CLR_DARK7, 777, 777, 777);
            init_color(CLR_DARK8, 888, 888, 888);

            init_pair(CLR_BLACK,    CLR_BLACK,      CLR_BLACK);
            init_pair(CLR_RED,      CLR_RED,        CLR_BLACK);
            init_pair(CLR_GREEN,    CLR_GREEN,      CLR_BLACK);
            init_pair(CLR_YELLOW,   CLR_YELLOW,     CLR_BLACK);
            init_pair(CLR_BLUE,     CLR_BLUE,       CLR_BLACK);
            init_pair(CLR_MAGENTA,  CLR_MAGENTA,    CLR_BLACK);
            init_pair(CLR_CYAN,     CLR_CYAN,       CLR_BLACK);
            init_pair(CLR_WHITE,    CLR_WHITE,      CLR_BLACK);

            init_pair(CLR_DARK1,    CLR_DARK1,      CLR_BLACK);
            init_pair(CLR_DARK2,    CLR_DARK2,      CLR_BLACK);
            init_pair(CLR_DARK3,    CLR_DARK3,      CLR_BLACK);
            init_pair(CLR_DARK4,    CLR_DARK4,      CLR_BLACK);
            init_pair(CLR_DARK5,    CLR_DARK5,      CLR_BLACK);
            init_pair(CLR_DARK6,    CLR_DARK6,      CLR_BLACK);
            init_pair(CLR_DARK7,    CLR_DARK7,      CLR_BLACK);
            init_pair(CLR_DARK8,    CLR_DARK8,      CLR_BLACK);
        }
    }
}

impl Console for CursesConsole {
    fn new() -> Self {
        let window = initscr();
        let ww = window.get_max_x() as usize;
        let wh = window.get_max_x() as usize;

        let mut res = Self {
            window: window,
            message: None,
            frame: 0,
            glitcher: Glitcher::new(ww, wh),
            use_colors: false,
        };

        raw();
        noecho();
        start_color();
        res.window.keypad(true);
        res.window.nodelay(true);

        res.init_colors();

        res
    }

    fn render<G>(&mut self, mut game: G) where G: Game {
        loop {
            let res;

            if self.frame == 0 {
                res = InputType::FirstFrame;
            }
            else {
                res = match self.window.getch() {
                    Some(Input::Character(ch)) => InputType::Char(ch),

                    Some(Input::KeyLeft) => InputType::Arrow(ArrowType::Left),
                    Some(Input::KeyRight) => InputType::Arrow(ArrowType::Right),
                    Some(Input::KeyUp) => InputType::Arrow(ArrowType::Up),
                    Some(Input::KeyDown) => InputType::Arrow(ArrowType::Down),

                    Some(Input::KeyF0) => InputType::Func(0),
                    Some(Input::KeyF1) => InputType::Func(1),
                    Some(Input::KeyF2) => InputType::Func(2),
                    Some(Input::KeyF3) => InputType::Func(3),
                    Some(Input::KeyF4) => InputType::Func(4),
                    Some(Input::KeyF5) => InputType::Func(5),
                    Some(Input::KeyF6) => InputType::Func(6),
                    Some(Input::KeyF7) => InputType::Func(7),
                    Some(Input::KeyF8) => InputType::Func(8),
                    Some(Input::KeyF9) => InputType::Func(9),
                    Some(Input::KeyF10) => InputType::Func(10),
                    Some(Input::KeyF11) => InputType::Func(11),
                    Some(Input::KeyF12) => InputType::Func(12),
                    Some(Input::KeyF13) => InputType::Func(13),
                    Some(Input::KeyF14) => InputType::Func(14),
                    Some(Input::KeyF15) => InputType::Func(15),

                    Some(Input::KeyBackspace) =>
                        InputType::Key(KeyType::Backspace),
                    Some(Input::KeyEnter) =>
                        InputType::Key(KeyType::Enter),
                    Some(Input::KeyHome) =>
                        InputType::Key(KeyType::Home),
                    Some(Input::KeyEnd) =>
                        InputType::Key(KeyType::End),

                    Some(Input::KeyResize) => {
                        resize_term(0, 0);

                        let nww = self.window.get_max_x();
                        let nwh = self.window.get_max_y();
                        self.glitcher.resize(nww as usize, nwh as usize);
                        InputType::Resize(nww as u32, nwh as u32)
                    },

                    _ => InputType::Char('a'),
                };
            }

            let uis = game.react(res);
            for ui in uis.into_iter() {
                match ui {
                    Ui::Message(_, _, s) => {
                        self.message = Some(BottomMessage::new(s));
                    },
                }
            }

            let buffer = game.gen_buffer();
            buffer.as_ref().map(|buf| {
                self.draw(buf);
            });

            self.draw_message();

            self.glitcher.update();
            self.glitcher.render(&self.window);

            self.frame += 1;

            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn get_width(&self) -> usize {
        self.window.get_max_x() as usize
    }

    fn get_height(&self) -> usize {
        self.window.get_max_y() as usize
    }
}

impl Drop for CursesConsole {
    fn drop(&mut self) {
        endwin();
    }
}
