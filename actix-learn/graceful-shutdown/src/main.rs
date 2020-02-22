use actix_web::{get, web, App, HttpServer, Responder};
use futures::executor;
use std::{sync::mpsc, thread};

#[get("/")]
async fn index() -> impl Responder {
    "Hello"
}

#[get("/stop")]
async fn stop(stopper: web::Data<mpsc::Sender<()>>) -> impl Responder {
    stopper.send(()).unwrap();
    "stop"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let (tx,rx) = mpsc::channel::<()>();

    let server = HttpServer::new(move || {
        App::new()
            .data(tx.clone())
            .service(index)
            .service(stop)
    })
    .bind("localhost:8086")?
    .run();

    let srv = server.clone();

    thread::spawn(move || {
        rx.recv().unwrap();

        executor::block_on(srv.stop(true));
    });

    server.await
}
