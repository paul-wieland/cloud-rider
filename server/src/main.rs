mod connection_manager;
mod domain;
mod mavlink;
mod websocket;

use crate::connection_manager::ConnectionManager;
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(websocket_handler));
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::serve(tokio::net::TcpListener::bind(address).await.unwrap(), app)
        .await
        .unwrap();
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket: WebSocket| async move {
        let mut client = ConnectionManager::new(socket);
        client.run().await;
    })
}
