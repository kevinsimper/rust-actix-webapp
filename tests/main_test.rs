use actix_web::{test, web, App, HttpServer, Responder};
extern crate rust_actix_webapp;
use rust_actix_webapp::routes;

#[actix_rt::test]
async fn test_index() {
    let srv = test::start(|| App::new().service(web::resource("/").route(web::get().to(routes::index))));

    let req = srv.get("/");
    let mut response = req.send().await.unwrap();
    let rawbody = response.body().await.unwrap();
    let message = String::from_utf8(rawbody.to_vec()).unwrap();
    println!("{}", message);
    assert!(message == String::from("Hello from Rust"));
    assert!(response.status().is_success());
}
