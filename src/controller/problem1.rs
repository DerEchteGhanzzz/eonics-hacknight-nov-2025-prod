use crate::problems::problem1;
use actix_web::{get, post, web, HttpResponse, Responder};
use crate::controller::css::CSS;
use crate::controller::receiver;

#[get("/problem1/definition")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::get_problem()))
}

#[get("/problem1/input")]
pub async fn get_input() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::get_input()))
}



#[post("/problem1/solve")]
pub async fn solve(payload: web::Payload) -> impl Responder {
    let body = receiver::handle_post::<i32>(payload).await;
    if body.is_err() {
        return HttpResponse::NotAcceptable().body(format!("{:?}", body));
    }
    println!("{:?}", body);
    let ans = &body.unwrap().to_string();
    if problem1::answer(ans) {
        return HttpResponse::Ok().body(format!(
            "{ans} is correct!"
        ));
    }
    HttpResponse::BadRequest().body(format!("{} is the wrong Answer, please try again", ans))
}

#[get("/problem1/answer")]
pub async fn answer() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::solve()))
}

#[get("/problem1/code")]
pub async fn code() -> impl Responder {
    HttpResponse::Ok().body(format!("
        {}
        <body>
        {}
        </body>
        ", CSS, problem1::get_code().replace("\n", "<br/>"))
    )
}