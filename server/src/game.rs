use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub uuid::Uuid);
impl serde::Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Game {
    active_turn: UserId,
}

impl Game {
    pub fn new(turn: UserId) -> Self {
        Self { active_turn: turn }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameId(pub Uuid);

impl serde::Serialize for GameId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.0)
    }
}