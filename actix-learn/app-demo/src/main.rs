use actix_web::{web, HttpServer, App, middleware};
use actix_files as fs;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                fs::Files::new("/", "./webapp/build/").index_file("index.html")
            )
    })
        .bind("localhost:8090")?
        .run()
        .await
}

