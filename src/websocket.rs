use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use crate::models::TokenDTO;

pub type WsSender = broadcast::Sender<TokenDTO>;

pub async fn start_websocket_server(
    port: u16,
) -> (WsSender, impl std::future::Future<Output = ()>) {
    let (tx, _) = broadcast::channel::<TokenDTO>(100);
    let tx_for_filter = tx.clone();

    let tx_filter = warp::any().map(move || tx_for_filter.clone());

    let websocket_route =
        warp::path("ws")
            .and(warp::ws())
            .and(tx_filter)
            .map(|ws: warp::ws::Ws, tx: WsSender| {
                ws.on_upgrade(move |socket| handle_websocket_client(socket, Arc::new(tx)))
            });

    let server = warp::serve(websocket_route).run(([127, 0, 0, 1], port));

    (tx, server)
}

async fn handle_websocket_client(ws: WebSocket, tx: Arc<WsSender>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();

    // Enviar mensajes al cliente
    tokio::spawn(async move {
        while let Ok(token) = rx.recv().await {
            if let Ok(msg) = serde_json::to_string(&token) {
                if ws_tx.send(Message::text(msg)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Mantener la conexión viva
    while let Some(result) = ws_rx.next().await {
        if result.is_err() {
            break;
        }
    }
}
