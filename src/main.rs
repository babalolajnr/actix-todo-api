// Start server and file watcher with
// systemfd --no-pid -s http::5000 -- cargo watch -x run
mod api_error;
pub mod auth;
mod db;
mod schema;
pub mod user;

use std::{env, io};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use log::info;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| {
        App::new()
            .configure(user::init_routes)
            .service(web::scope("/auth").configure(auth::init_routes))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST not set");
            let port = env::var("PORT").expect("PORT not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!(
        "Starting server at http://{}:{}",
        server.addrs()[0].ip(),
        server.addrs()[0].port()
    );

    server.run().await
}
