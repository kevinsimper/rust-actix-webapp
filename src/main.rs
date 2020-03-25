use actix_web::{web, App, HttpServer};
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").route(web::get().to(routes::index)))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
