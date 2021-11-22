use std::sync::Arc;

use axum::extract::Extension;
use axum::Json;
use uuid::Uuid;

use crate::{
    game::{Game, GameId},
    AppState, ClientInfo, UserId,
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
    client_info: ClientInfo,
    connection_state: ConnectionState,
}
/// Give a user a userid. We are implicitly trusting all clients not to lie about their userid.
/// Conversely, I guess I could just use their IP as the userid, but since there's other aspects
/// of poor design where I'm already planning on just trusting the client, I don't care.
pub(crate) async fn connect(Extension(state): Extension<Arc<AppState>>) -> Json<ConnectResponse> {
    let mut waiting_users = state.waiting_clients.write().await;
    let other_player = match waiting_users.pop() {
        Some(user) => user,

        // If there's no waiting players, tell the client to wait for a player.
        None => {
            let client_player = ClientInfo {
                user_id: UserId(Uuid::new_v4()),
                game_id: GameId(Uuid::new_v4()),
            };
            waiting_users.push(client_player);
            return Json(ConnectResponse {
                client_info: client_player,
                connection_state: ConnectionState::WaitingForGame,
            });
        }
    };
    let client_player = ClientInfo {
        user_id: UserId(Uuid::new_v4()),
        game_id: other_player.game_id,
    };

    let ClientInfo {
        user_id: player_turn,
        ..
    } = if rand::random() {
        client_player
    } else {
        other_player
    };

    state.games.insert(
        client_player.game_id,
        Game::new(player_turn, [client_player.user_id, other_player.user_id]),
    );

    Json(ConnectResponse {
        client_info: client_player,
        connection_state: ConnectionState::new_joined_game(player_turn),
    })
}
