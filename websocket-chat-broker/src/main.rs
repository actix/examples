#[macro_use]
extern crate log;

use actix::fut;
use actix::prelude::*;
use actix_broker::BrokerIssue;
use actix_files::Files;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod server;
use server::*;

fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsChatSession::default(), &req, stream)
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
        // issue_sync comes from having the `BrokerIssue` trait in scope.
        self.issue_system_sync(leave_msg, ctx);
        // Then send a join message for the new room
        let join_msg = JoinRoom(
            room_name.to_owned(),
            self.name.clone(),
            ctx.address().recipient(),
        );

        WsChatServer::from_registry()
            .send(join_msg)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.id = id;
                    act.room = room_name;
                }

                fut::ok(())
            })
            .spawn(ctx);
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
            })
            .spawn(ctx);
    }

    fn send_msg(&self, msg: &str) {
        let content = format!(
            "{}: {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            msg
        );
        let msg = SendMessage(self.room.clone(), self.id, content);
        // issue_async comes from having the `BrokerIssue` trait in scope.
        self.issue_system_async(msg);
    }
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.join_room("Main", ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!(
            "WsChatSession closed for {}({}) in room {}",
            self.name.clone().unwrap_or_else(|| "anon".to_string()),
            self.id,
            self.room
        );
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
        debug!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();
                if msg.starts_with('/') {
                    let mut command = msg.splitn(2, ' ');
                    match command.next() {
                        Some("/list") => self.list_rooms(ctx),
                        Some("/join") => {
                            if let Some(room_name) = command.next() {
                                self.join_room(room_name, ctx);
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
                        Some("/name") => {
                            if let Some(name) = command.next() {
                                self.name = Some(name.to_owned());
                                ctx.text(format!("name changed to: {}", name));
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", msg)),
                    }
                    return;
                }
                self.send_msg(msg);
            }
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => {}
        }
    }
}

fn main() -> std::io::Result<()> {
    let sys = actix::System::new("websocket-broker-example");
    simple_logger::init_with_level(log::Level::Info).unwrap();

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/ws/").to(chat_route))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .start();

    info!("Started http server: 127.0.0.1:8080");
    sys.run()
}
