use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result};

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;
    let server_address = "127.0.0.1";

    println!("Listening on port {port}");
    println!("http://127.0.0.1:{port}");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind((server_address, port))?
        .run()
        .await
}
