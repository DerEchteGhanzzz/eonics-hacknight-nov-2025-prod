use crate::problems::problem3;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::{css::CSS, receiver}, structures::params::Answer};

#[get("/problem3/definition")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_problem()))
}

#[get("/problem3/input")]
pub async fn get_input() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_input()))
}

#[post("/problem3/solve")]
pub async fn solve(payload: web::Payload) -> impl Responder {
    let body = receiver::handle_post::<i32>(payload).await;
    if body.is_err() {
        return HttpResponse::NotAcceptable().body(format!("{:?}", body));
    }
    println!("{:?}", body);
    if problem3::answer(&body.unwrap().to_string()) {
        return HttpResponse::Ok().body(format!(
            "Success!"
        ));
    }
    HttpResponse::BadRequest().body(format!("{}", wrong_answer()))
}

#[get("/problem3/answer")]
pub async fn answer() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem3::solve()))
}

#[get("/problem3/code")]
pub async fn code() -> impl Responder {
    HttpResponse::Ok().body(format!("
        {}
        <body>
        {}
        </body>
        ", CSS, problem3::get_code().replace("\n", "<br/>"))
    )
}

fn wrong_answer() -> String {
    String::from(
        "Wrong Answer: That's not going to cut it."
    )
}