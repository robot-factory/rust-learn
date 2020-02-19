use actix_web::{HttpServer, App, web, HttpResponse};

fn scope_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/scope")
                .route(web::get().to(|| HttpResponse::Ok().body("/scope")))
                // .route("/a", web::get().to(|| HttpResponse::Ok().body("/a")))
        )
        .service(
            web::scope("/scope2")
                .route("", web::get().to(|| HttpResponse::Ok().body("/scope2")))
        );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api")
            .route(web::get().to(|| HttpResponse::Ok().body("/api")))
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/v1").configure(scope_config))
    })
    .bind("127.0.0.1:8084")?
    .run()
    .await
}