#[macro_use]
extern crate actix;
extern crate actix_web;
extern crate actix_broker;
extern crate futures;
extern crate rand;

use actix::prelude::*;
use actix::fut;
use actix_web::server::HttpServer;
use actix_web::{fs, http, ws, App, Error, HttpRequest, HttpResponse};
use actix_broker::BrokerIssue;
use futures::Future;

mod server;
use server::*;

fn chat_route(req: &HttpRequest<()>) -> Result<HttpResponse, Error> {
    ws::start(req, WsChatSession::default())
}

#[derive(Default)]
struct WsChatSession {
    id: usize,
    room: String,
    name: Option<String>,
}

impl WsChatSession {
    fn join_room(&mut self, room_name: &str, ctx: &mut ws::WebsocketContext<Self>) {
        let room_name = room_name.to_owned();
        // First send a leave message for the current room
        let leave_msg = LeaveRoom(self.room.clone(), self.id);
        self.issue_sync(leave_msg, ctx);
        // Then send a join message for the new room
        let join_msg = JoinRoom(
            room_name.to_owned(),
            self.name.clone(),
            ctx.address().recipient());

        WsChatServer::from_registry()
            .send(join_msg)
            .map_err(|_| ())
            .into_actor(self)
            .map(|id, act, _| {
                act.id = id;
                act.room = room_name;
            })
            .wait(ctx);
    }

    fn list_rooms(&mut self, ctx: &mut ws::WebsocketContext<Self>) {
        WsChatServer::from_registry()
            .send(ListRooms)
            .into_actor(self)
            .then(|res, _, ctx| {
                if let Ok(rooms) = res {
                    for room in rooms {
                        ctx.text(room);
                    }
                }
                fut::ok(())
            }).wait(ctx);
    }

    fn send_msg(&self, msg: &str) {
        let content = format!("{}: {}",
                              self.name.clone().unwrap_or("anon".to_string()),
                              msg);
        let msg = SendMessage(self.room.clone(), self.id, content);
        self.issue_async(msg);
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.join_room("Main", ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WsChatSession stopped for {}({}) in room {}",
            self.name.clone().unwrap_or("anon".to_string()),
            self.id,
            self.room);
    }
}

impl Handler<ChatMessage> for WsChatSession {
    type Result = (); 

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsChatSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Text(text) => {
                let m = text.trim();
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => self.list_rooms(ctx),
                        "/join" => {
                            if v.len() == 2 {
                                self.join_room(v[1], ctx);
                                ctx.text("joined");
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                                ctx.text(format!("name changed to: {}", v[1]));
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", m)),
                    }
                    return;
                } 
                self.send_msg(m);
            }
            ws::Message::Close(_) => {
                ctx.stop();
            },
            _ => {},
        }
    }
}

fn main() {
    let sys = actix::System::new("websocket-broker-example");

    HttpServer::new(move || {
        App::new()
            .resource("/", |r| r.method(http::Method::GET).f(|_| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/websocket.html")
                    .finish()
            }))
            .resource("/ws/", |r| r.route().f(chat_route))
            .handler("/static/", fs::StaticFiles::new("static/").unwrap())
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
