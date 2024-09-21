use actix_web::{
    get,
    http::header::ContentType,
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use tibia::ProtocolGame;

mod model;
mod tibia;

#[get("/status")]
async fn status(query: web::Query<ProtocolGame>) -> impl Responder {
    let ProtocolGame { ip, port } = query.0;
    let tibia = ProtocolGame { ip, port };

    match tibia.server_status() {
        Ok(server) => HttpResponse::Ok()
            .insert_header(ContentType::json())
            .json(server),
        Err(err) => {
            let (code, message) = err;
            HttpResponse::NotFound()
                .insert_header(ContentType::json())
                .json(model::error::Error {
                    code,
                    message: message.to_string(),
                })
        }
    }
}

#[actix_web::main]
#[allow(unused_must_use)]
async fn main() {
    let server = HttpServer::new(|| App::new().service(status)).bind(("0.0.0.0", 3000));

    if let Ok(app) = server {
        println!("Server online!");
        app.run().await;
    }
}
