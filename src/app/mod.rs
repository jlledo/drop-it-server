use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web_actors::ws;
use std::process::Command;

mod command;
mod parser;
mod socket;
use socket::ControlSocket;

const SOCKET_ADDRESS: &str = "0.0.0.0:6916";

pub fn run_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(ws_index)
            // shutdown route
            .service(shutdown)
    })
    .bind(SOCKET_ADDRESS)?
    .run();

    Ok(())
}

#[get("/control")]
async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    log::info!("{:?}", r);
    let res = ws::start(ControlSocket::new(), &r, stream);
    log::info!("{:?}", res);
    res
}

#[get("/shutdown")]
async fn shutdown() -> Result<String> {
    Command::new("cmd")
        .args(&["/C", "shutdown -s"])
        .output()
        .expect("failed to shutdown");

    Ok(String::from("Bye!"))
}
