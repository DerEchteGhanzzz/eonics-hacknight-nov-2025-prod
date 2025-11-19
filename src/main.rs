use std::io;
use actix_web::{App, HttpServer};

use crate::problems::parser;
mod structures;
mod controller;
mod problems;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();
    let answer1 = problems::problem1::solve();
    let answer2 = problems::problem2::solve();
    let answer3 = problems::problem3::solve();

    parser::write_file("./src/output_files/answer1.txt", &format!("{answer1}"));
    parser::write_file("./src/output_files/answer2.txt", &format!("{answer2:?}"));
    parser::write_file("./src/output_files/answer3.txt", &format!("{answer3}"));

    HttpServer::new(|| App::new()
    .service(controller::pizza_place::index)
    .service(controller::pizza_place::open)
    .service(controller::pizza_place::menu)
    .service(controller::pizza_place::order_pizza)
    .service(controller::pizza_place::get_ingredients)
    .service(controller::problem1::get_problem)
    .service(controller::problem1::get_input)
    .service(controller::problem1::solve)
    .service(controller::problem1::answer)
    .service(controller::problem1::code)
    .service(controller::problem2::get_problem)
    .service(controller::problem2::get_input)
    .service(controller::problem2::solve)
    .service(controller::problem2::answer)
    .service(controller::problem2::code)
    .service(controller::problem3::get_problem)
    .service(controller::problem3::get_from_to)
    .service(controller::problem3::get_locations)
    .service(controller::problem3::solve)
    .service(controller::problem3::answer)
    .service(controller::problem3::code)
    )
        // .bind_openssl("192.168.178.253:80", builder)?
        // .bind("192.168.178.253:80")?
        .bind("localhost:80")?
        .run()
        .await
}