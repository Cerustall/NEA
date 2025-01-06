use futures::{Future, StreamExt, FutureExt};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio::sync::mpsc;
use crate::{RegisterRequest, RegisterResponse, Clients, Client, Event, TopicsRequest};
//use warp::ws::{Message, WebSocket};
//use warp::Reply;
//use warp::body::json;
//use warp::http::StatusCode;
use warp::{http::StatusCode, Rejection, reply::json, Reply, ws::{Message, WebSocket}};
use uuid::Uuid;
use serde_json::from_str;

pub async fn register_handler<E>(body: RegisterRequest, clients: Clients) -> Result<impl Reply, E> {
    // Create new user id for connecting client
    let user_id = body.user_id;

    // Create new UUID for connecting client
    let uuid = Uuid::new_v4().simple().to_string();

    register_client(uuid.clone(), user_id, clients).await;
    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
    }))
}

// Creates a client with empty user_id generated above, default topics (cats), and an empty sender
// When the clients container is next available for editing, this new client is added to it
async fn register_client(id: String, user_id: usize, clients: Clients) {
    // Async hashmap => lock needs to be aquired, and awaited (since its a future) 
    clients.lock().await.insert(
        id,
        Client {
            user_id,
            topics: vec![String::from("cats")],
            sender: None,
        },
    );
    // When lock goes out of scope its dropped, allowing other actors to access Clients contianer
}

pub fn health_handler<StatusCode>() -> impl Future<Output = Result<impl Reply, warp::http::StatusCode>> {
    futures::future::ready(Ok(StatusCode::OK))
}

// Handles client disconnecting
pub async fn unregister_handler<E>(id: String, clients: Clients) -> Result<impl Reply, E> {
    // Same concept as register client, just inverse
    clients.lock().await.remove(&id);
    Ok(StatusCode::OK)
}

// Allows clients to websocket endpoint after registering
pub async fn ws_handler<Rejection>(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply, warp::Rejection> {
    // Checks if given client exists
    let client = clients.lock().await.get(&id).cloned();
    match client {
        // If client exists, upgrade to websocket connection
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, id, clients, c))),
        // 404 if client does not exist
        None => Err(warp::reject::not_found()),
    }
}

// Handles client connections, sending, and receiving messages
pub async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    // Splits websocket into sender and receiver
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    // Create unbounded multiple producer single consumer (MPSC) channel
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    // Client_sender is the sender object on the client
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    // Formwards recieved messages to sender
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    // Updates client
    // Client passed to fn recieves sender part of channel
    client.sender = Some(client_sender);
    // Clients structure updated
    clients.lock().await.insert(id.clone(), client);

    // No messaging errors, we can log client as connected
    println!("{} connected", id);

    // Waits for incoming messages on the receiver channel
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            // Receive errors logged
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
                }
        };

    // Recieved messages are forwaded to client_msg
    client_msg(&id, msg, &clients).await;
    }

    // This part of the function is only accessible in case of error
    // In case of error, client is disconnected and removed from list of clients
    clients.lock().await.remove(&id);
    println!("{} disconnected", id);
}

// Handles incoming messages
async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    // Logs incoming message and sender
    println!("received message from {}: {:?}", id, msg);
    
    // Only interested in string messages
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    // Special case for 'messages' being pings
    if message == "ping" || message == "ping\n" {
        return;
    }

    // Special case for 'messages' being topics requests
    // Topics requests allows users to change what topics they're interested in
    let topics_req: TopicsRequest = match from_str(&message) {
        Ok(v) => v,
        // Errors logged if they occur
        Err(e) => {
        eprintln!("error while parsing message to topics request: {}", e);
        return;
        }
    };

    // In the case of a topics request, wait for clients structre to be available
    // then alter the topics of the relevant client as necessary
    let mut locked = clients.lock().await;
    match locked.get_mut(id) {
        Some(v) => {
        v.topics = topics_req.topics;
        }
    None => return,
    };
}

// Handles broadcasting messages to all users interested in the topics of the message
pub async fn publish_handler<E>(body: Event, clients: Clients) -> Result<impl Reply, E> {
    // Iterates over clients structure, filtering out clients who arent the specified user/s,
    // and sending the message to the rest
    clients
        .lock()
        .await
        .iter_mut()
        .filter(|(_, client)| match body.user_id {
            Some(v) => client.user_id == v,
            None => true,
        })
        .filter(|(_, client)| client.topics.contains(&body.topic))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::text(body.message.clone())));
            }
        });
  Ok(StatusCode::OK)
}
