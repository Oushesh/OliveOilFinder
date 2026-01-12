use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};


async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
    //return HttpResponse::Ok().finish();
}


async fn greet(req: HttpRequest) -> impl Responder {
    //function takes request and outputs hello
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


#[tokio::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //Registering the health_check route
            .route("/health_check",web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}