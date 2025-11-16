use crate::problems::problem2;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::css::CSS, structures::params::Answer};

#[get("/problem2")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_problem()))
}

#[get("/input2")]
pub async fn get_input() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_input()))
}

#[get("/solve2")]
pub async fn solve(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem2::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!(
            r#"{}
            <body>
                <h2>
                    Alright, that's a lot of pizza! I think we'll be good for now.<br/>
                    We only have one more <a href="/problem3">issue</a>, can you handle this one?<br/>
                    Because some have told me this one is <a href="https://en.wikipedia.org/wiki/NP-hardness"><b>Notoriously Pretty Hard</b></a>?
                <h2>
            </body>"#,
            CSS
        ));
    }
    HttpResponse::BadRequest().body(format!("{}{}", CSS, wrong_answer()))
}

#[get("/answer2")]
pub async fn answer() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem2::solve()))
}

#[get("/code2")]
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
        r#"
            <p>
                <b>Wrong Answer</b>
                Are you sure that is right? I think you're missing something.<br/>
                remember you need to give back a formatted list please: [0, 0, 0, 0, 0]
            </p>
        "#
    )
}