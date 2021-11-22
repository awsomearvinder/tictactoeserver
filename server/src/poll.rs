use crate::AppState;

use axum::extract::{Extension, Query};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::{GameId, UserId};

#[derive(serde::Serialize)]
pub(crate) struct PollResponse {
    current_game_state: crate::Game,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GameInfo {
    user_id: UserId,
    game_id: GameId,
}

pub(crate) async fn poll(
    Extension(state): Extension<Arc<AppState>>,
    Query(client_info): Query<GameInfo>,
) -> Json<PollResponse> {
    dbg!(client_info);
    Json(PollResponse {
        current_game_state: todo!(),
    })
}
