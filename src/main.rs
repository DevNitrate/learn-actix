use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{any}")]
async fn any(any_path: web::Path<String>) -> impl Responder {
    format!("endpoint: {any_path}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Listening on port {port}");

    HttpServer::new(|| App::new().service(any))
        .bind(("127.0.0.1", port)).unwrap()
        .run()
        .await
}
