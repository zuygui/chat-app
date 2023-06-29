use std::{collections::{HashMap}, sync::{atomic::{AtomicUsize, Ordering}, Arc}};
use serde::{Serialize, Deserialize};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    /// Client name
    pub name: Option<String>,
    /// Client session
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Name of the user
    #[serde(skip_serializing_if = "Option::is_none", rename = "username")]
    pub name: Option<String>,
    /// Peer message
    #[serde(rename = "message")]
    pub msg: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat session.
///
/// Implementation is very naïve.
#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
            visitor_count,
        }
    }

    /// Send message to all users in chat
    fn send_message(&self, message: ClientMessage, skip_id: usize) {
        log::debug!("Sending message to {} users", self.sessions.len());
        for id in &self.sessions.keys().cloned().collect::<Vec<usize>>() {
            if *id != skip_id {
                if let Some(addr) = self.sessions.get(id) {
                    let message = serde_json::to_string(&message).unwrap();
                    addr.do_send(Message (message.to_owned()));
                }
            }
        }
    }

    fn send_server_message(&self, message: &str) {
        self.send_message(ClientMessage { id: 0, name: None, msg: message.to_string() }, 0);
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
/// 
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Someone joined");
        
        // notify all users in same room
        self.send_server_message("Someone joined");

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);


        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_server_message(&format!("Total visitors : {count}"));

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        log::info!("Someone disconnected");

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            self.send_server_message("Someone disconnected");
        }
    }
}

/// Handler for ClientMessage message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        log::info!("Got message from client");
        let msg_id = msg.id.clone();
        self.send_message(msg, msg_id);
    }
}