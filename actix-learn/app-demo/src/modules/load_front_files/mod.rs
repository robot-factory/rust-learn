use actix_web::{web, Result};
use actix_files as fs;

async fn frontend_index() -> Result<fs::NamedFile> {
    Ok(actix_files::NamedFile::open("./webapp/build/index.html")?)
}


pub fn start(app: &mut web::ServiceConfig) {
    app
        .service(
            fs::Files::new("/", "./webapp/build/")
                .index_file("index.html")
                .default_handler(web::get().to(frontend_index))
        );
}
