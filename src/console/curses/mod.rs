pub mod messages;

use console::{ Console, InputType, ArrowType, KeyType };
use console::curses::messages::bottom::BottomMessage;
use game::Game;
use tile::Tile;
use tile::base::Base;
use tile::style::Style;
use ui::Ui;
use utils::rnd_lt;

use pancurses::*;
use rand::prelude::*;

use std::{thread, time};

pub struct CursesConsole {
    window: Window,
    message: Option<BottomMessage>,
    first_frame: bool,
}

impl CursesConsole {
    fn apply_style(&self, style: &Style) {
        //  first turn off all attributes
        self.window.attrset(A_NORMAL);

        if style.intersects(Style::STANDOUT) {
            //  for some reason A_STANDOUT doesn't compile
            //  self.window.attron(A_STANDOUT);
        }

        if style.intersects(Style::BOLD) {
            self.window.attron(A_BOLD);
        }

        if style.intersects(Style::UNDERLINE) {
            self.window.attron(A_UNDERLINE);
        }

        if style.intersects(Style::DIM) {
            self.window.attron(A_DIM);
        }
    }

    fn draw(&self, buffer: &Vec<Tile>) {
        let mut curr_style = Style::default();

        self.window.mv(0, 0);
        for tile in buffer.iter() {
            let base = &tile.base;
            let cover = &tile.cover;
            let style = &tile.style;

            if curr_style != *style {
                self.apply_style(style);
                curr_style = *style;
            }

            let mut chance: usize = 2000;
            let ch = match base {
                &Base::Void => {
                    chance = 0;
                    ' '
                },

                &Base::Player => 'o',
                &Base::Ground => '.',
                &Base::Wall => 'X',
                &Base::Water => '=',
                &Base::Message(ref s) => '?',
                &Base::Door(ref x) => 'd',
            };

            self.window.addch(Self::glit_ch(ch as u64, chance));
        }
    }

    fn draw_message(&self, msg: &BottomMessage) {
        let ww = self.get_width();
        let wh = self.get_height();

        let text = msg.get_text();
        let mut num_lines = text.len() / (ww - 4);
        if text.len() % (ww - 4) > 0 {
            num_lines += 1;
        }

        let sx = 0;
        let sy = wh - 4 - num_lines;

        self.window.mv(sy as i32, sx as i32);
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
                    _ => ' ' as u64,
                };

                self.window.addch(Self::glit_ch(ch, 2000));
            }
        }

        for line in 0..num_lines {
            let end = match line == num_lines - 1 {
                true => text.len() % (ww - 4),
                false => (line + 1) * (ww - 4),
            };

            self.window.mvaddstr(
                (sy + 2 + line) as i32,
                (sx + 2) as i32,
                &text[line * (ww - 4)..end]
            );
        }
    }

    fn glit_ch(ch: u64, chance: usize) -> u64 {
        match chance > 0 && rnd_lt::<usize>(chance) == 0 {
            true => Self::rnd_ch(),
            false => ch,
        }
    }

    fn rnd_ch() -> u64 {
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
            _ => ' ' as u64,
        }
    }
}

impl Console for CursesConsole {
    fn new() -> Self {
        let res = Self {
            window: initscr(),
            message: None,
            first_frame: true,
        };

        raw();
        noecho();
        res.window.keypad(true);
        res.window.nodelay(true);

        res
    }

    fn render<G>(&mut self, mut game: G) where G: Game {
        loop {
            let res;

            if self.first_frame {
                self.first_frame = false;
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
                        InputType::Resize(
                            self.window.get_max_x() as u32,
                            self.window.get_max_y() as u32,
                            )
                    },

                    _ => InputType::Char('a'),
                };
            }

            let uis = game.react(res);
            for ui in uis.into_iter() {
                match ui {
                    Ui::Message(s) => {
                        self.message = Some(BottomMessage::new(s));
                    },
                }
            }

            let buffer = game.gen_buffer();
            buffer.as_ref().map(|buf| {
                self.window.clear();
                self.draw(buf);
            });

            self.message.as_ref().map(|msg| {
                self.draw_message(msg);
            });

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
