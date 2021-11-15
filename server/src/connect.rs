use std::sync::Arc;

use axum::extract::Extension;
use axum::Json;
use uuid::Uuid;

use crate::{
    game::{Game, GameId},
    AppState, UserId,
};

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind")]
enum ConnectionState {
    WaitingForGame,
    JoinedGame { turn: UserId },
}
impl ConnectionState {
    fn new_joined_game(turn: UserId) -> Self {
        Self::JoinedGame { turn }
    }
}

#[derive(serde::Serialize)]
pub struct ConnectResponse {
    user_id: UserId,
    connection_state: ConnectionState,
}
/// Give a user a userid. We are implicitly trusting all clients not to lie about their userid.
/// Conversely, I guess I could just use their IP as the userid, but since there's other aspects
/// of poor design where I'm already planning on just trusting the client, I don't care.
pub(crate) async fn connect(Extension(state): Extension<Arc<AppState>>) -> Json<ConnectResponse> {
    let client_player = UserId(Uuid::new_v4());
    let mut waiting_users = state.waiting_users.write().await;

    let other_player = match waiting_users.pop() {
        Some(user) => user,

        // If there's no waiting players, tell the client to wait for a player.
        None => {
            waiting_users.push(client_player);
            return Json(ConnectResponse {
                user_id: client_player,
                connection_state: ConnectionState::WaitingForGame,
            });
        }
    };

    let player_turn = if rand::random() {
        client_player
    } else {
        other_player
    };

    state
        .games
        .insert(GameId(Uuid::new_v4()), Game::new(player_turn));

    Json(ConnectResponse {
        user_id: client_player,
        connection_state: ConnectionState::new_joined_game(player_turn),
    })
}
