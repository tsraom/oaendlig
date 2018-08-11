#[derive(PartialEq, Eq, Clone)]
pub enum Base {
    Void,
    Player,

    Ground,
    Wall,
    Water,
    Message(String),
    Door(usize),
}

impl Default for Base {
    fn default() -> Self {
        Base::Ground
    }
}

