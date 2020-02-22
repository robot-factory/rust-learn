use actix_web::{web, App, HttpServer, HttpResponse};
use actix_rt::System;
use std::sync::mpsc;
use std::thread;

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     let one = HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
//     });
//     let two = HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(|| HttpResponse::Ok().body("/2")))
//     });
//     one.bind("localhost:8085")?.run().await;
//     two.bind("localhost:8086")?.run().await
// }

// fn main() {
//     let sys = System::new("http-server");
//     let one = HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
//     });
//     one.bind("localhost:8085").unwrap().run();
//     sys.run();
// }

// #[actix_rt::main]
// async fn main() {
//     let (tx, rx) = mpsc::channel();

//     let handler = thread::spawn(move || -> std::io::Result<()> {
//         let sys = System::new("http-server");

//         let srv = HttpServer::new(|| {
//             App::new().route("/", web::get().to(|| HttpResponse::Ok().body("/")))
//         })
//         .bind("localhost:8087")?
//         .shutdown_timeout(60)
//         .run();

//         let _ = tx.send(srv);
//         sys.run()
//     });

//     let srv = rx.recv().unwrap();

//     srv.pause().await;

//     srv.resume().await;

//     // srv.stop(true).await;

//     handler.join().unwrap();
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .workers(4)
    .bind("127.0.0.1:8089")?
    .run()
    .await
}
