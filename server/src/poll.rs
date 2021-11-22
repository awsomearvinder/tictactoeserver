use crate::AppState;

use axum::body::{Bytes, Full};
use axum::extract::{Extension, Query};
use axum::http::Response;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;

use crate::{GameId, UserId};

#[derive(serde::Serialize)]
#[serde(tag = "kind")]
pub(crate) enum PollResponse {
    ActiveGame { game: crate::Game },
    WaitingForGame,
}

pub(crate) enum PollError {
    BadGameId(GameId),
    BadUserId(UserId),
}
impl IntoResponse for PollError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;
    fn into_response(self) -> Response<Self::Body> {
        match self {
            Self::BadGameId(id) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"kind": "InvalidGameID", "gameId": id})),
            ),
            Self::BadUserId(id) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"kind": "InvalidUserId", "userId": id})),
            ),
        }
        .into_response()
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct GameInfo {
    user_id: UserId,
    game_id: GameId,
}

pub(crate) async fn poll(
    Extension(state): Extension<Arc<AppState>>,
    Query(client_info): Query<GameInfo>,
) -> Result<Json<PollResponse>, PollError> {
    if let Some(game_state) = state.games.get(&client_info.game_id) {
        if game_state.players.iter().all(|&n| n != client_info.user_id) {
            Err(PollError::BadUserId(client_info.user_id))
        } else {
            Ok(Json(PollResponse::ActiveGame { game: *game_state }))
        }
    } else if state
        .waiting_clients
        .read()
        .await
        .iter()
        .any(|&n| n.user_id == client_info.user_id)
    {
        Ok(Json(PollResponse::WaitingForGame))
    } else {
        Err(PollError::BadGameId(client_info.game_id))
    }
}
