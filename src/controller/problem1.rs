use crate::problems;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::css::CSS, structures::params::Answer};
use crate::controller::receiver;

#[get("/problem1/definition")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problems::problem1::get_problem()))
}

#[get("/problem1/input")]
pub async fn get_input() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problems::problem1::get_input()))
}



#[post("/problem1/solve")]
pub async fn solve(payload: web::Payload) -> impl Responder {
    let body = receiver::handle_post::<i32>(payload).await;
    if body.is_err() {
        return HttpResponse::NotAcceptable().body(format!("{:?}", body));
    }
    println!("{:?}", body);
    if problems::problem1::answer(&body.unwrap().to_string()) {
        return HttpResponse::Ok().body(format!(
            "Success!"
        ));
    }
    HttpResponse::BadRequest().body(format!("{}", wrong_answer()))
}

#[get("/problem1/answer")]
pub async fn answer() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problems::problem1::solve()))
}

#[get("/problem1/code")]
pub async fn code() -> impl Responder {
    HttpResponse::Ok().body(format!("
        {}
        <body>
        {}
        </body>
        ", CSS, problems::problem1::get_code().replace("\n", "<br/>"))
    )
}

fn wrong_answer() -> String {
    String::from(
        "Wrong Answer, please try again"
    )
}