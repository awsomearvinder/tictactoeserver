use crate::GameId;
use crate::UserId;

#[derive(Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(crate) struct ClientInfo {
    pub(crate) user_id: UserId,
    pub(crate) game_id: GameId,
}
