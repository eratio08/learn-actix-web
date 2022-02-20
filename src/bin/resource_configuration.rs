use actix_web::{get, guard, web, App, HttpRequest, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show usrs")
}

#[get("/show/{id}")]
async fn user_details(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User details: {}", path.into_inner().0))
}

#[get("/a/{v1}/{v2}")]
async fn match_information(req: HttpRequest) -> HttpResponse {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v1").parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    HttpResponse::Ok().body(format!("{} {} {} {}", v1, v2, v3, v4))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                web::resource("/user/{nale}")
                    .name("user_details")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(|| HttpResponse::Ok())),
            )
            .service(
                web::resource("/bla")
                    .route(web::route().guard(guard::Get()).to(|| HttpResponse::Ok())),
            )
            .service(
                web::scope("/users")
                    .service(show_users)
                    .service(user_details),
            )
            .service(match_information)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
