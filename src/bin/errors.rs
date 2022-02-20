use actix_web::{
    dev::HttpResponseBuilder,
    error, get,
    http::{header, StatusCode},
    middleware, web, App, HttpServer,
};
use derive_more::{Display, Error};
use env_logger::Logger;
use log::info;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError { name: "test" })
}

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "ValidationError on filed: {}", field)]
    ValidationError { field: String },
    #[display(fmt = "An inernal error occured. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

type MyResult<T> = Result<T, UserError>;

#[get("/tests")]
async fn tests() -> MyResult<String> {
    let err = UserError::InternalError {};
    info!("{}", err);
    Err(err)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = middleware::Logger::default();

        App::new()
            .wrap(logger)
            .service(tests)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
