use std::io;
use actix_web::{App, HttpServer};
mod structures;
mod controller;
mod presentation;
mod problems;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new()
    .service(controller::pizza_place::index)
    .service(controller::pizza_place::open)
    .service(controller::pizza_place::menu)
    .service(controller::pizza_place::get_ingredients)
    .service(controller::pizza_place::get_problem1)
    .service(controller::pizza_place::solve1)
    .service(controller::pizza_place::answer1)
    .service(controller::pizza_place::get_problem2)
    .service(controller::pizza_place::solve2)
    .service(controller::pizza_place::answer2)
    .service(controller::pizza_place::get_problem3)
    .service(controller::pizza_place::solve3)
    .service(controller::pizza_place::answer3)
    )
        // .bind_openssl("192.168.178.253:80", builder)?
        .bind("192.168.178.253:80")?
        .run()
        .await
}