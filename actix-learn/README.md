# actix learn
里程碑
1. 学习actix-web的常见用法，完成realworld项目。
2. 阅读actix-web源码，深入了解actix。

## 基础
最基础的代码 -> simplest
```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    "simple word"
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
2. 通过App创建应用；
3. 基础的相应函数返回一个 `impl Responder` ， 最简单的创建方式是直接返回字符串， 可操作性大一点的是`HttpResponse::Ok().body("Hello world!")`
4. 配置路由的方式有两种，集中式和分散式，集中式是通过App实例的route函数，分布式是利用宏， 通过service函数来载入服务；
5. `#[actix_rt::main]`就是是并发的简写，目前还不确定，资料太少。

## 路由划分
一般来讲一个后台程序都有很多功能模块，编写程序的时候相同的模块一般有同样开头的路由，并且复用一部分功能，因此路由划分是一个常见的需求。
实例 -> route_scope
```rust
...
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
```
结果是`/app1/`返回`hi for app1`，`/app2/`返回`hi for app2`，`/app3/`返回`hi for app3`
1. web::scope 生成的是一个service；
2. web::scope 能导入已有的service；
3. 这里的service比较偏向 controller 的概念；