use std::{net::SocketAddr, sync::Arc};

use axum::{AddExtensionLayer, Router};
use dashmap::DashMap;
use tokio::sync::RwLock;

mod connect;
mod game;
pub(crate) use game::{Game, GameId, UserId};

#[derive(Default)]
struct AppState {
    // RwLock, since we *rarely* add or remove userid's (hopefully), let's optimize for
    // the readers case. :)
    // tl;dr with a Mutex<> you need to acquire the lock to get a read only *or* a write
    // only reference, but a RwLock will let you get as many read only's as you want, as long as
    // no writer exists. Conversely, a writer must wait until no readers exist.
    waiting_users: RwLock<Vec<UserId>>,
    games: DashMap<GameId, Game>,
}

/// Entry point for the server, this runs on the runtime we started in main.
async fn async_main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // This state is shared across all routes, hence why it's atomically reference counted.
    // (We're using a multithreaded runtime.)
    // Realistically I could probably just leak it and then get rid of the ref-count that way too
    // But I don't really care. :) Both work fine.
    let state = Arc::new(AppState::default());

    let router = Router::new()
        .route("/connect", axum::routing::get(connect::connect))
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

/// Spawns a runtime and starts the server process on it.
fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(async_main());
}
