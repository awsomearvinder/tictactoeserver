use axum::body::{Bytes, Full};
use axum::extract::{Extension, Query};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::game::{GameId, UserId};
use crate::AppState;

#[derive(Serialize)]
pub struct MoveResponse;

pub enum MoveError {
    InvalidIndex,
    SpotAlreadyTaken { x: usize, y: usize },
    InvalidUserId { id: UserId },
    InvalidGameId { id: GameId },
    NotYourTurn,
}

impl IntoResponse for MoveError {
    type Body = Full<Bytes>;

    type BodyError = std::convert::Infallible;

    fn into_response(self) -> http::Response<Self::Body> {
        match self {
            Self::InvalidIndex => (
                StatusCode::BAD_REQUEST,
                Json(json!({"kind": "InvalidIndex"})),
            ),
            Self::SpotAlreadyTaken { x, y } => (
                StatusCode::BAD_REQUEST,
                Json(json!({"kind": "SpotAlreadyTaken", "x": x, "y": y})),
            ),
            MoveError::InvalidUserId { id } => (
                StatusCode::BAD_REQUEST,
                Json(json!({"kind": "InvalidUserId", "user_id": id})),
            ),
            MoveError::InvalidGameId { id } => (
                StatusCode::BAD_REQUEST,
                Json(json!({"kind": "InvalidGameId", "game_id": id})),
            ),
            MoveError::NotYourTurn => (
                StatusCode::BAD_REQUEST,
                Json(json!({"kind": "NotYourTurn"})),
            ),
        }
        .into_response()
    }
}

#[derive(Deserialize, Debug)]
pub struct MoveInfo {
    x: usize,
    y: usize,
    user_id: UserId,
    game_id: GameId,
}

pub(crate) async fn make_move(
    Extension(state): Extension<Arc<AppState>>,
    Query(move_info): Query<MoveInfo>,
) -> Result<Json<MoveResponse>, MoveError> {
    let game = match state.games.get(&move_info.game_id) {
        Some(v) => v,
        None => {
            return Err(MoveError::InvalidGameId {
                id: move_info.game_id,
            })
        }
    };
    let mut game = game.lock().unwrap();
    //TODO: Move these checks into game.board.make_move()
    if game.players.iter().all(|&n| n != move_info.user_id) {
        return Err(MoveError::InvalidUserId {
            id: move_info.user_id,
        });
    }
    if game.active_turn != move_info.user_id {
        return Err(MoveError::NotYourTurn);
    }
    if move_info.x >= 3 || move_info.y >= 3 {
        return Err(MoveError::InvalidIndex);
    }
    if game.board[move_info.y][move_info.x].is_some() {
        return Err(MoveError::SpotAlreadyTaken {
            x: move_info.x,
            y: move_info.y,
        });
    }
    game.make_move(move_info.x, move_info.y).unwrap();
    Ok(Json(MoveResponse))
}
