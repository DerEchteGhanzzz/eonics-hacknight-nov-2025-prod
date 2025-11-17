use crate::problems::problem2;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::{css::CSS, receiver}, structures::params::Answer};

#[get("/problem2/definition")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_problem()))
}

#[get("/problem2/input")]
pub async fn get_input() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_input()))
}

#[post("/problem2/solve")]
pub async fn solve(payload: web::Payload) -> impl Responder {
    println!("Here");
    let body = receiver::handle_post::<Vec<i32>>(payload).await;
    if body.is_err() {
        println!("{:?}", body);
        return HttpResponse::NotAcceptable().body(format!("{:?}", body));
    }
    println!("{:?}", body);
    if problem2::answer(&format!("{:?}", body.unwrap())) {
        return HttpResponse::Ok().body(format!(
            "Success!"
        ));
    }
    HttpResponse::BadRequest().body(format!("{}", wrong_answer()))
}

#[get("/problem2/answer")]
pub async fn answer() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem2::solve()))
}

#[get("/problem2/code")]
pub async fn code() -> impl Responder {
    HttpResponse::Ok().body(format!("
        {}
        <body>
        {}
        </body>
        ", CSS, problem2::get_code().replace("\n", "<br/>"))
    )
}

fn wrong_answer() -> String {
    String::from(
        "Wrong Answer, please try again"
    )
}