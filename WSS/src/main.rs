// Help largely comes from:
// https://blog.logrocket.com/build-websocket-server-with-rust/

/*
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use futures::*;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use serde_derive::{Deserialize, Serialize};
use serde_json::*;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use uuid::*;
use warp::ws::Message;
use warp::*;
use crate::hyper::StatusCode;
use crate::StatusCode;
use crate::ws:Message;
*/

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};
pub use serde_derive::{Deserialize, Serialize};

/* Part of previous code
#[derive(Clone)]
struct AppState {
    connections: Arc<Mutex<HashMap<String, UnboundedSender<String>>>>,
}
*/

// Defines standard result type
type Result<T> = std::result::Result<T, Rejection>;

// Package that contains connected clients (to be initialised)
// Hashmap maps clients UUID to their username
// Mutex ensures only one accessor at a time, to prevent overwrite errors 
// Arc allows type to be passed between threads safely
type Clients = Arc<Mutex<HashMap<String, Client>>>;

// THe client
#[derive (Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

// EXPLAIN THIS STRUCT
#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterRequest {
    user_id: usize,
}

// EXPLAIN THIS STRUCT
#[derive(serde::Deserialize, serde::Serialize)]
pub struct RegisterResponse {
    url: String,
}

// Event e.g message, broadcast
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Event {
    topic: String,
    user_id: Option<usize>,
    message: String,
}

// EXPLAIN THIS STRUCT
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TopicsRequest {
    topics: Vec<String>,
}

mod handler;
//mod ws;


// Main
#[tokio::main]
async fn main() {
    // Initialise client container
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    
    // Initialise routes

    // Inidicates service up
    let health_route = warp::path!("health").and_then(handler::health_handler);

    // Registers/deregisters clients
    let register = warp::path("register");
    //Register
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler) // Following section is deregister
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));
    
    // Broadcast events
    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    // WS endpoint
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(handler::ws_handler);

    // Defins all routes
    let routes = health_route
        .or(register_routes)
        .or(ws_route)
        .or(publish)
        .with(warp::cors().allow_any_origin());
    
    // Serves above routes on given IPv4 and port (Currently loopback for debug and testing)
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

// Inserts clients into their container? I think?
fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
  warp::any().map(move || clients.clone())
}



/* Previous program
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
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
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
*/
