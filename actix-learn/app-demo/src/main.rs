use actix_web::{web, HttpServer, App, middleware,HttpResponse, HttpRequest, Result};
use actix_files as fs;

mod app;
mod modules;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let server = app::new_server();
    server.await
}

