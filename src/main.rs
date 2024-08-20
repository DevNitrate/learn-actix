use actix_files as fs;
use fs::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{file}.html")]
async fn html(html_file: web::Path<String>) -> actix_web::Result<impl Responder> {
    let f: std::io::Result<NamedFile> = NamedFile::open(format!("public/{}.html", html_file));
    if f.is_err() {
        return Ok(NamedFile::open("public/404.html").unwrap());
    }
    Ok(f.unwrap())
}

#[get("/{file}.css")]
async fn css(css_file: web::Path<String>) -> actix_web::Result<impl Responder> {
    let f: std::io::Result<NamedFile> = NamedFile::open(format!("public/{}.css", css_file));
    if f.is_err() {
        return Ok(NamedFile::open("public/404.html").unwrap());
    }
    Ok(f.unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Listening on port {port}");

    HttpServer::new(|| App::new().service(html).service(css))
        .bind(("0.0.0.0", port)).unwrap()
        .run()
        .await
}
