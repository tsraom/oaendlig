use uuid::Uuid;

pub struct Player {
    room: Uuid,
    x: usize,
    y: usize,
}

impl Player {
    pub fn new(room: Uuid, x: usize, y: usize) -> Self {
        Self {
            room: room,
            x: x,
            y: y,
        }
    }

    pub fn nil() -> Self {
        Self {
            room: Uuid::nil(),
            x: 0,
            y: 0,
        }
    }

    pub fn get_room(&self) -> &Uuid {
        &self.room
    }

    pub fn get_x(&self) -> usize {
        self.x
    }
     
    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn set_room(&mut self, room: Uuid) {
        self.room = room;
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }
}
