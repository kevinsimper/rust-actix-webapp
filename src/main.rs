use actix_web::{test, web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    format!("Hello from Rust")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[actix_rt::test]
async fn test_index() {
    let srv = test::start(|| App::new().service(web::resource("/").route(web::get().to(index))));

    let req = srv.get("/");
    let mut response = req.send().await.unwrap();
    let rawbody = response.body().await.unwrap();
    let message = String::from_utf8(rawbody.to_vec()).unwrap();
    println!("{}", message);
    assert!(message == String::from("Hello from Rust"));
    assert!(response.status().is_success());
}
