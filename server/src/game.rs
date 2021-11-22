use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct UserId(pub uuid::Uuid);
impl serde::Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

//TODO: GameId and UserId share the same implementation for this. Macro?
struct UserIdVisitor;

impl<'de> serde::de::Visitor<'de> for UserIdVisitor {
    type Value = UserId;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a properly formed UUID")
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let user_id = if let Ok(v) = Uuid::from_str(v) {
            v
        } else {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ));
        };
        Ok(UserId(user_id))
    }
}

impl<'de> serde::Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(UserIdVisitor)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Game {
    pub active_turn: UserId,
    pub board: [[Option<UserId>; 3]; 3],
    pub players: [UserId; 2],
}

impl Game {
    pub fn new(turn: UserId, players: [UserId; 2]) -> Self {
        assert!(players.iter().any(|&n| n == turn));
        Self {
            active_turn: turn,
            board: [[None; 3]; 3],
            players,
        }
    }
    pub fn make_move(&mut self, x: usize, y: usize) -> Option<()> {
        self.board[y][x] = Some(self.active_turn);
        let &new_player_turn = self.players.iter().find(|&&n| n != self.active_turn)?;
        self.active_turn = new_player_turn;
        Some(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GameId(pub Uuid);

impl serde::Serialize for GameId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.0)
    }
}

struct GameIdVisitor;

impl<'de> serde::de::Visitor<'de> for GameIdVisitor {
    type Value = GameId;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a properly formed UUID")
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let user_id = if let Ok(v) = Uuid::from_str(v) {
            v
        } else {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self,
            ));
        };
        Ok(GameId(user_id))
    }
}

impl<'de> serde::Deserialize<'de> for GameId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(GameIdVisitor)
    }
}
