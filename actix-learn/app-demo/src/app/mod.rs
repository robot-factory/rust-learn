use actix_web::{web, HttpServer, App, middleware, HttpResponse, HttpRequest, Responder, Result, error};
use actix_files as fs;
use actix_server::Server;
use actix_web::FromRequest;

use serde::{Deserialize, Serialize};
use rocksdb::{DB, Options};

use crate::modules::load_front_files;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MyObj {
    name: String,
}

async fn json_handler(info: web::Json<MyObj>) -> String {
    format!("Welcome {}!", info.name)
}

async fn put(info: web::Path<(String,String)>) -> Result<String> {
    Ok(format!("put {}:{} sucess!", info.0, info.1))
}

pub fn new_server() -> Server {
    
    let bind_address: String = String::from("localhost:8090");

    let path = "db_data";
//    let db = DB::open_default(path).unwrap();

    HttpServer::new(move|| {
        App::new()
//            .data(DB::open_default(path).unwrap())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(web::resource("put/{key}/{value}")
                        .route(web::get().to(put)))
                    .service(web::resource("echo")
                        .app_data(web::Json::<MyObj>::configure(|cfg| {
                        cfg.limit(4096).content_type(|_| true).into()
                    })).route(web::post().to(json_handler))))
            .configure(load_front_files::start)

    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
}
