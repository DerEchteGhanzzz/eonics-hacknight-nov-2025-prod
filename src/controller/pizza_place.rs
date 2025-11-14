use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::structures::{pizzas::available_pizzas, params::{OrderPizza, Answer}};
use crate::problems::{problem1, problem2, problem3};

#[get("/open")]
pub async fn open() -> impl Responder {
    HttpResponse::Ok().body("We're open! Come in! Check our menu via /menu, and check /ingredients for what each pizza contains.\n 
                            To order a pizza, you can call /order-pizza with your pizza of choice from the menu.")
}

#[get("/menu")]
pub async fn menu() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", available_pizzas()))
}

#[get("/ingredients")]
pub async fn get_ingredients(req: HttpRequest) -> impl Responder {
    let request = web::Query::<OrderPizza>::from_query(req.query_string());
    if request.is_err() {
        return HttpResponse::BadRequest().body("params usage:\n\tname: Name of pizza; check /menu for our the menu.");
    }
    let order = &request.unwrap().name;
    HttpResponse::Ok().body(format!("{:?}", order.ingredients()))
}

#[get("/problem1")]
pub async fn get_problem1() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::get_problem()))
}

#[get("/solve1")]
pub async fn solve1(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem1::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!("Oh, very good. Thank you for checking! That streamlines everything nicely"));
    }
    HttpResponse::BadRequest().body("Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?")
}

#[get("/answer1")]
pub async fn answer1() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem1::solve()))
}

#[get("/problem2")]
pub async fn get_problem2() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_problem()))
}

#[get("/solve2")]
pub async fn solve2(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem2::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!("Oh, very good. Thank you for checking! That streamlines everything nicely"));
    }
    HttpResponse::BadRequest().body("Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?")
}

#[get("/answer2")]
pub async fn answer2() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem2::solve()))
}

#[get("/problem3")]
pub async fn get_problem3() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_problem()))
}

#[get("/solve3")]
pub async fn solve3(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem3::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!("Very well. I'll send him on his way and he'll be with you shortly!"));
    }
    HttpResponse::BadRequest().body("Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?")
}

#[get("/answer3")]
pub async fn answer3() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem3::solve()))
}