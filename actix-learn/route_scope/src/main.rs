use actix_web::{web, HttpServer, App, Responder};
use actix_web::get;

async fn index() -> impl Responder {
    "hi for app1"
}

async fn index2() -> impl Responder {
    "hi for app2"
}

#[get("/")]
async fn index3() -> impl Responder {
    "hi for app3"
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/app1").route("/", web::get().to(index)),
            )
            .service(
                web::scope("/app2").route("/", web::get().to(index2))
            )
            .service(
                web::scope("/app3").service(index3)
            )
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}