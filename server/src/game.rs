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
    active_turn: UserId,
}

impl Game {
    pub fn new(turn: UserId) -> Self {
        Self { active_turn: turn }
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
