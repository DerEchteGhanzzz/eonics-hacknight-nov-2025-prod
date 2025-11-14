use std::io;
use actix_web::{App, HttpServer};
mod structures;
mod controller;
mod presentation;
mod problems;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new()
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
        .bind("127.0.0.1:25565")?
        .run()
        .await
}