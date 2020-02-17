use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(index3)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}