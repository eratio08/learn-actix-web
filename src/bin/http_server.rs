use actix_web::{self, web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(|| HttpResponse::Ok())))
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
