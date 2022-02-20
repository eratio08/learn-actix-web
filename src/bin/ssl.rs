use std::{fs::File, io::BufReader};

use actix_web::{get, App, HttpRequest, HttpServer, Responder};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cert_file = &mut BufReader::new(File::open("example.com+5.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("example.com+5-key.pem").unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        eprintln!("Cloud not locate PKCS 8 private keys.");
        std::process::exit(1)
    }

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0))
        .unwrap();

    println!("Staring https server: 127.0.0.1:8888");
    HttpServer::new(|| App::new().service(index))
        .bind_rustls("127.0.0.1:8888", config)?
        .run()
        .await
}
