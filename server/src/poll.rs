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
pub(crate) struct PollResponse {
    current_game_state: crate::Game,
}

pub(crate) enum PollError {
    BadGameId(GameId),
}
impl IntoResponse for PollError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;
    fn into_response(self) -> Response<Self::Body> {
        match self {
            Self::BadGameId(id) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"errorKind": "InvalidGameID", "gameId": id})),
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
    let game_state = if let Some(v) = state.games.get(&client_info.game_id) {
        v
    } else {
        return Err(PollError::BadGameId(client_info.game_id));
    };
    Ok(Json(PollResponse {
        current_game_state: *game_state,
    }))
}
