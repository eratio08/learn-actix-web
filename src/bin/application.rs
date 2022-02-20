use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefix creates a scope
            web::scope("/app")
                // becomes /app/index.html
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
