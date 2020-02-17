# actix learn
里程碑
1. 学习actix-web的常见用法，完成realworld项目。
2. 阅读actix-web源码，深入了解actix。

## 基础
最基础的代码
```rust
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
```
1. 通过HttpServer创建服务器并启动；
2. 通过App创建服务；
3. 基础的相应函数返回一个 `impl Responder` ， 最简单的创建方式是 `HttpResponse::Ok().body("Hello world!")`
4. 配置路由的方式有两种，集中式和分散式，集中式是通过App实例的route函数，分布式是利用宏；