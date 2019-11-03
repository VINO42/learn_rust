use actix_web::{web, App, HttpResponse,HttpRequest, HttpServer, Responder};
use actix_web::get;
use log::{info, trace, warn};
use std::io::Write;
use chrono::Local;
use std::cell::Cell;

struct AppState{
    counter:Cell<usize>
}
#[macro_use]
fn index(info: web::Path<(u32, String)>) -> impl Responder {
    info!("default path in  {},{}!", info.1, info.0);
    format!("Hello {}! id:{}", info.1, info.0)
}



#[get("/hello")]
fn index2( ) -> String {
    info!("hello actix");
    HttpResponse::Ok().body("hello actix")
}

fn index3() -> impl Responder {
    info!("router in !");
    HttpResponse::Ok().body("hello router")
}

fn main() -> std::io::Result<()> {
    //init logger
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S.%f"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");

  HttpServer::new({
        || App::new().service(
            //表示prefix scope
            web::scope("app").route("/{id}/{name}/index.html", web::get().to(index))
        ).service(index2).route("/r", web::get().to(index3))
    }).bind("127.0.0.1:9999")?.run()
}

