mod comm_base;
mod download_file;
mod entity;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use reqwest::Client;
use ulid::Ulid;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> Result<impl Responder> {
    let obj = entity::t_material_object::Model {
        id: Ulid::new().to_string(),
        create_time: None,
        delete_tag: None,
        update_time: None,
        description: None,
        name: Option::from(name.to_string()),
        state: None,
        tags: None,
        r#type: None,
        object_local_path: None,
        source_obj_local_path: None,
        source_obj_url: None,
        target_obj_url: None,
    };
    Ok(web::Json(obj))
}

#[get("/download")]
async fn download() -> impl Responder {
    let url = "http://172.16.3.52:9000/publish-web/test/Sku_AJ.glb";
    // let url = "https://releases.ubuntu.com/20.04/ubuntu-20.04.3-desktop-amd64.iso";

    let re = download_file::download_file(&Client::new(), url, "Sku_AJ.glb").await.unwrap();

    HttpResponse::Ok().body("Hello world!")
}

async fn index() -> impl Responder {
    "Hello world! index"
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index_app_state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {}!", app_name) // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_name)
            .service(download)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/app")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/index.html", web::get().to(index)),
            )
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web app"),
            }))
            .service(index_app_state)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
