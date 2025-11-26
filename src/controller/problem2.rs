use crate::problems::problem2;
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::{controller::{css::CSS, receiver}};

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
    let ans = format!("{:?}", body.unwrap());
    if problem2::answer(&ans) {
        return HttpResponse::Ok().body(format!(
            "{ans} is correct!"
        ));
    }
    HttpResponse::BadRequest().body(format!("{} is the wrong Answer, please try again", ans))
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