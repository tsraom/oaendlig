use console::{ InputType, ArrowType };
use tile::Tile;
use tile::base::Base;
use game::Game;
use ui::Ui;

mod room;

use game::endless::room::Room;
use game::endless::room::square::SquareRoom;

use uuid::Uuid;
use rand::prelude::*;

use std::iter;
use std::collections::HashMap;
use std::mem;

use utils::{ rnd_within, rnd_lt, rnd_string };

struct PlayerInfo {
    room: Uuid,
    x: usize,
    y: usize,
}

impl PlayerInfo {
    fn new(room: Uuid, x: usize, y: usize) -> PlayerInfo {
        PlayerInfo {
            room: room,
            x: x,
            y: y,
        }
    }

    fn nil() -> PlayerInfo {
        PlayerInfo {
            room: Uuid::nil(),
            x: 0,
            y: 0,
        }
    }
}

type RoomDoor = (Uuid, usize);

pub struct EndlessGame {
    rooms: HashMap<Uuid, Box<Room>>,

    //  if (A, B) is in links, then (B, A) is in links
    links: HashMap<RoomDoor, RoomDoor>,
    player: PlayerInfo,

    //  should be None when buffer is invalid, i.e. doesn't match size
    buffer: Option<Vec<Tile>>,
    buf_width: usize,
    buf_height: usize,
}

impl EndlessGame {
    fn init(
        &mut self,
        num_rooms: usize,
        num_links: usize,
    ) {
        //  we use these uuids to build rooms later
        let mut uuids: Vec<Uuid> = Vec::new();
        uuids.resize_with(
            num_rooms as usize, || Uuid::new_v4()
            );

        //  the kth element in this vector represents how many doors the kth
        //  room needs
        let mut door_cnts: Vec<usize> =
            iter::repeat(0).take(num_rooms).collect();

        //  first build links, then build rooms
        if num_rooms > 1 {
            for _ in 0..num_links {
                //  pick 2 different rooms
                let room0 = random::<usize>() % num_rooms;
                let mut room1: usize = room0;
                while room1 == room0 {
                    room1 = random::<usize>() % num_rooms;
                }

                let uuid0 = uuids[room0].clone();
                let uuid1 = uuids[room1].clone();
                let door0 = door_cnts[room0];
                let door1 = door_cnts[room1];

                self.links.insert(
                    (uuid0, door0), (uuid1, door1)
                    );

                self.links.insert(
                    (uuid1.clone(), door1), (uuid0.clone(), door0)
                    );

                door_cnts[room0] += 1;
                door_cnts[room1] += 1;
            }
        }

        for x in 0..num_rooms {
            let uuid = uuids[x].clone();

            let room_width = rnd_within::<usize>(10..15);
            let room_height = rnd_within::<usize>(10..15);

            self.rooms.insert(
                uuid,
                Box::new(SquareRoom::new(
                    uuid.clone(),
                    door_cnts[x],
                    room_width,
                    room_height
                    )
                ));
        }

        let uuid = uuids[random::<usize>() % uuids.len()].clone();

        self.player = PlayerInfo::new(
            uuid,
            rnd_lt(self.rooms.get(&uuid).unwrap().get_width() - 2) + 1,
            rnd_lt(self.rooms.get(&uuid).unwrap().get_height() - 2) + 1
        );
    }
}

impl Game for EndlessGame {
    fn new(buf_width: usize, buf_height: usize) -> Self {
        let mut res = Self {
            rooms: HashMap::new(),
            links: HashMap::new(),
            player: PlayerInfo::nil(),
            buffer: None,
            buf_width: buf_width,
            buf_height: buf_height,
        };

        let num_rooms = rnd_within::<usize>(5..11);
        let num_links = rnd_within::<usize>(10..21);

        res.init(num_rooms, num_links);
        res
    }

    fn react(&mut self, input: InputType) -> Vec<Ui> {
        let mut res = Vec::new();

        match input {
            InputType::FirstFrame => {
                res.push(Ui::Message(
                    format!("This is {} reporting from room {}.", rnd_string(10), self.player.room.simple())
                ));
            },

            InputType::Arrow(arrow) => {
                let room = self.rooms.get(&self.player.room).unwrap();
                let x = self.player.x;
                let y = self.player.y;

                let mut nx = x;
                let mut ny = y;
                match arrow {
                    ArrowType::Left => { nx = x - 1; },
                    ArrowType::Right => { nx = x + 1; },
                    ArrowType::Up => { ny = y - 1; },
                    ArrowType::Down => { ny = y + 1; },
                };

                match room.get_tile(nx, ny).map(|tile| &tile.base) {
                    Some(&Base::Ground) => {
                        self.player.x = nx;
                        self.player.y = ny;
                    },

                    Some(&Base::Door(idx)) => {
                        let &(to_room, to_door) =
                            self.links.get(&(self.player.room, idx)).unwrap();

                        self.player.room = to_room.clone();
                        self.player.x =
                            rnd_lt(self.rooms.get(&to_room).unwrap().get_width() - 2) + 1;
                        self.player.y = 
                            rnd_lt(self.rooms.get(&to_room).unwrap().get_height() - 2) + 1;

                        res.push(Ui::Message(
                            format!("This is {} reporting from room {}.", rnd_string(10), to_room.simple())
                        ));
                    },

                    _ => (),
                }
            },

            InputType::Resize(w, h) => {
                self.buffer.take();
                self.buf_width = w as usize;
                self.buf_height = h as usize;
            },

            _ => (),
        };

        res
    }

    fn gen_buffer(&mut self) -> Option<&Vec<Tile>> {
        let room = self.rooms.get(&self.player.room).unwrap();
        let mut tiles: Vec<Tile> = Vec::new();

        let px = self.player.x as i32;
        let py = self.player.y as i32;
        let bw = self.buf_width as i32;
        let bh = self.buf_height as i32;

        let sx = px - bw / 2;
        let sy = py - bh / 2;
        let ex = px + (bw - bw / 2);
        let ey = py + (bh - bh / 2);

        for y in sy..ey {
            for x in sx..ex {
                let ux = x as usize;
                let uy = y as usize;;

                match x == px && y == py {
                    true => {
                        tiles.push(Tile::player());
                        continue;
                    },
                    false => (),
                }

                match (x >= 0 && y >= 0, room.get_tile(ux, uy)) {
                    (true, Some(ref t)) => { tiles.push((*t).clone()); },
                    _ => { tiles.push(Tile::void()); },
                }
            }
        }

        {
            mem::replace(&mut self.buffer, Some(tiles));
        }

        self.buffer.as_ref()
    }
}
