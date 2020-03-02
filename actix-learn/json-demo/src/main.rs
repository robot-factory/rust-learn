use serde_derive::{Serialize, Deserialize};
use actix_web::{web, App, HttpServer, Result, HttpRequest, Responder, HttpResponse};

#[derive(Deserialize, Serialize)]
struct Info {
    username: String,
}

/// deserialize `Info` from request's body
async fn index(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().json(info)
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

async fn index2(req: HttpRequest) -> Result<web::Json<MyObj>> {
    Ok(web::Json(MyObj {
        name: req.match_info().get("name").unwrap().to_string(),
    }))
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/index.html").route(
                web::post().to(index))
         ).service(
             web::resource("/{name}").route(
                 web::get().to(index2)
             )
         )
    })
    .bind("localhost:8091")?
    .run()
    .await
}