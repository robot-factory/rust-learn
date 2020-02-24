use actix_web::{web, HttpServer, App, middleware,HttpResponse, HttpRequest, Result};
use actix_files as fs;
use actix_server::Server;

use crate::load_front_files;

pub fn new_server() -> Server {
    let bind_address: String = String::from("localhost:8090");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/v1").route(web::get().to(|| HttpResponse::Ok().body("api"))))
            .configure(load_front_files::start)

    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
}
