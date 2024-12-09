use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

#[derive(Clone)]
struct AppState {
    connections: Arc<Mutex<HashMap<String, UnboundedSender<String>>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let state = AppState {
        connections: Arc::new(Mutex::new(HashMap::new())),
    };
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest_service("/", ServeDir::new("web"))
        .with_state(state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|ws| handle_socket(ws, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let username = socket.recv().await.unwrap().unwrap().into_text().unwrap();
    let (tx, mut rx) = mpsc::unbounded_channel();
    {
        info!("new connection {}", username);

        let mut connections = state.connections.lock().await;
        connections.insert(username.clone(), tx);
    }
    loop {
        tokio::select! {
            msg = socket.recv() => {
                let text = format!("{}: {}", username, msg.unwrap().unwrap().into_text().unwrap());
                info!("{:?}", text);
                let connections = state.connections.lock().await;
                for tx in connections.values() {
                    tx.send(text.clone()).unwrap();
                }
            },
            msg = rx.recv() => {
                info!("received mpsc message {:?}", msg);
                socket.send(Message::Text(msg.unwrap())).await.unwrap();
            }
        }
    }
}
