mod download_file;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/download")]
async fn download() -> impl Responder {
    let url="http://172.16.3.52:9000/publish-web/test/Sku_AJ.glb";
    // let url = "https://releases.ubuntu.com/20.04/ubuntu-20.04.3-desktop-amd64.iso";

    let re = download_file::download_file(&Client::new(), url, "Sku_AJ.glb")
        .await
        .unwrap();

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



