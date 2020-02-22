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
5. `#[actix_rt::main]`是一个宏，用于将异步函数调入`actix_rt::System`进行执行

## Handler
我现在有个疑问，actix是如何控制handler函数的入参的？编写的函数输入参数并没有限制，比如express的handler就限制入参是req，res，next三个，但actix完全没有规范，而且还有共享数据，那它分配请求时是怎么调用handler函数的？

## 路由匹配
1. `App.new().route("/path", web::get().to(index))`
2. #[get("/main")]

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

然而这样用于分离模块还是不够方便，需要更近一步。actix-web将路由配置以类似参数的形式导入
示例 -> config_scope
```rust 
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
```

## 应用共享状态
有时候我们需要再不同的`controller`之间共享状态， actix-web也同样提供了方案。
实例　－>　state_share
```rust
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
```
共享状态的数据直接注入响应的函数，之后最好详细了解下`Mutex`的使用方法。

注册共享数据有两种方式：
```rust

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {app_name: String::from("Actix-web"), counter: Mutex::new(0)})
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}

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
```

两者的测试结果是相同的， 后一种主要用于已有数据的导入， 虽然看起来好像比较麻烦，但实际应该比较方便使用。

## guard守卫
Guard用于对某个路由的访问添加限制，可以是一个返回值为布尔类型的函数。
```rust
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("www"))),
            )
            .service(
                web::scope("/")
                    .guard(guard::Header("Host", "users.rust-lang.org"))
                    .route("", web::to(|| HttpResponse::Ok().body("user"))),
            )
            .route("/", web::to(|| HttpResponse::Ok()))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
```

## 服务器管理
一般来讲，web框架里server的功能是将底层的http服务和业务逻辑结合起来。App主要负责业务逻辑，Server相关主要就是负责服务的管理了。运维会涉及比较多服务的管理。简单来讲就是需要控制服务器的运行状态。
最基本的功能是启动、暂停、重启、结束。actix-web也提供了对应的方法：　run(), pause(), resume(), stop()

### 优雅的暂停、重启、关机

```rust
#[actix_rt::main]
async fn main() {
    let (tx, rx) = mpsc::channel();

    let handler = thread::spawn(move || -> std::io::Result<()> {
        let sys = System::new("http-server");

        let srv = HttpServer::new(|| {
            App::new().route("/", web::get().to(|| HttpResponse::Ok().body("/")))
        })
        .bind("localhost:8087")?
        .shutdown_timeout(60)
        .run();

        let _ = tx.send(srv);
        sys.run()
    });

    let srv = rx.recv().unwrap();

    srv.pause().await;

    srv.resume().await;

    // srv.stop(true).await;

    handler.join().unwrap();
}
```

```rust
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
```
server管理的主要方式就是通过 mpsc::channel 获得server对象，在外部进行操作。
需要注意的是：
1. 启动多线程时要记得阻塞。


### 如何启动多个服务器
需要通过thread::spawn启动线程来启动不同的服务

### 多线程操作



## 中间件
中间件是一个很重要的概念