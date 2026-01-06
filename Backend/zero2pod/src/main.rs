/*
fn main() {
    println!("Hello, world!");
}
*/

//Our First Endpoint: A Basic health Check
//From Book zeor-to-production-in-rust-an-introduction book from the past.


use actix_web::{web, App, HttpRequest, HttpServer, Responder};
async fn greet(req: HttpRequest) -> impl Responder {
let name = req.match_info().get("name").unwrap_or("World");
format!("Hello {}!", &name)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {
HttpServer::new(|| {
App::new()
.route("/", web::get().to(greet))
.route("/{name}", web::get().to(greet))
})
.bind("127.0.0.1:8000")?
.run()
.await
}
