use crate::problems;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::css::CSS, structures::params::Answer};

#[get("/problem1")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problems::problem1::get_problem()))
}

#[get("/input1")]
pub async fn get_input() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problems::problem1::get_input()))
}

#[get("/solve1")]
pub async fn solve(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problems::problem1::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!(
            r#"{}
            <body>
                <h2>
                    Perfect! I think we might be able to fit them all!<br/>
                    Okay, while I go an do that, could you maybe look at <a href="/problem2">problem 2</a>?
                </h2>
            </body>"#,
            CSS
        ));
    }
    HttpResponse::BadRequest().body(format!("{}{:?}", CSS, wrong_answer()))
}

#[get("/answer1")]
pub async fn answer() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problems::problem1::solve()))
}

#[get("/code1")]
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
        r#"
            <p>
                <b>Wrong Answer</b>
                Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?
            </p>
        "#
    )
}