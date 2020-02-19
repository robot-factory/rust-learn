use actix_web::{HttpServer, App, web, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter +=1;
    format!("hello {} visited {}", app_name, counter)
}


// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .data(AppState {app_name: String::from("Actix-web"), counter: Mutex::new(0)})
//             .route("/", web::get().to(index))
//     })
//     .bind("127.0.0.1:8083")?
//     .run()
//     .await
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_data = AppState {app_name: String::from("Actix-web"), counter: Mutex::new(0)};
    let app_data = web::Data::new(app_data);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}