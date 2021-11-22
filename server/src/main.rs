use std::{net::SocketAddr, sync::Arc};

use axum::{AddExtensionLayer, Router};
use axum_server::tls_rustls::RustlsConfig;

use dashmap::DashMap;
use tokio::sync::RwLock;
use tower_http::cors::{any, CorsLayer};

mod client_info;
mod connect;
mod game;
mod mover;
mod poll;

pub(crate) use client_info::ClientInfo;
pub(crate) use game::{Game, GameId, UserId};

#[derive(Default)]
struct AppState {
    // RwLock, since we *rarely* add or remove userid's (hopefully), let's optimize for
    // the readers case. :)
    // tl;dr with a Mutex you need to acquire the lock to get a read only *or* a write
    // only reference, but a RwLock will let you get as many read only's as you want, as long as
    // no writer exists. Conversely, a writer must wait until no readers exist.
    waiting_clients: RwLock<Vec<ClientInfo>>,
    games: DashMap<GameId, std::sync::Mutex<Game>>,
}

/// Entry point for the server, this runs on the runtime we started in main.
async fn async_main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // This state is shared across all routes, hence why it's atomically reference counted.
    // (We're using a multithreaded runtime.)
    // Realistically I could probably just leak it and then get rid of the ref-count that way too
    // But I don't really care. :) Both work fine.
    let state = Arc::new(AppState::default());

    let config = RustlsConfig::from_pem_file("certs/cert.pem", "certs/key.pem")
        .await
        .unwrap();

    let router = Router::new()
        .route("/move", axum::routing::get(mover::make_move))
        .route("/connect", axum::routing::get(connect::connect))
        .route("/poll", axum::routing::get(poll::poll))
        .layer(CorsLayer::new().allow_origin(any()).allow_methods(any()))
        .layer(AddExtensionLayer::new(state));

    axum_server::bind_rustls(addr, config)
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
