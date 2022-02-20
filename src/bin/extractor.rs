use actix_web::{get, post, web, App, HttpServer, Result};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(friends)
            .service(query_handler)
            .service(json_handler)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")]
async fn index(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {} {}", friend, user_id))
}

#[get("/friends/{friend_id}/{name}")]
async fn friends(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {} {}!", info.friend_id, info.name))
}

#[derive(Deserialize)]
struct Info {
    friend_id: u32,
    name: String,
}

#[get("/query")]
async fn query_handler(query: web::Query<QueryParam>) -> Result<String> {
    Ok(format!("Query {}", query.test))
}

#[derive(Deserialize)]
struct QueryParam {
    test: String,
}

#[post("/")]
async fn json_handler(body: web::Json<Body>) -> Result<String> {
    Ok(format!("{}", body.field_a))
}

#[derive(Deserialize)]
struct Body {
    field_a: String,
}
