use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("say Hi!")
}

async fn index4() -> impl Responder {
    HttpResponse::Ok().body("Hello app!")
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| {
    //     App::new()
    //         .route("/", web::get().to(index))
    //         .route("/again", web::get().to(index2))
    // }).bind("127.0.0.1:8088")?.run().await

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(index3)
            .service(web::resource("/again").to(index2))
            .service(web::scope("/app").route("/1", web::get().to(index4)))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
