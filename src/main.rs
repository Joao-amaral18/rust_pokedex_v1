use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer};

use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
     "project_name" :"Pokedex",
     "pokemons":[
         {
             "name":"bulbassaur",
             "image_path":"/static/image/001.png"
         },
         {
             "name":"charmander",
             "image_path":"/static/image/004.png"
         },
         {
             "name":"bulbassaur",
             "image_path":"/static/image/007.png"
         }
     ]
    });

    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;
    let server_address = "127.0.0.1";
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Listening on port {port}");
    println!("http://127.0.0.1:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind((server_address, port))?
    .run()
    .await
}
