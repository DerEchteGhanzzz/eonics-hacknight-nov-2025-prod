use crate::problems::problem3;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::css::CSS, structures::params::Answer};

#[get("/problem3")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_problem()))
}

#[get("/input3")]
pub async fn get_input() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_input()))
}

#[get("/solve3")]
pub async fn solve(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem3::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!("Very well. I'll send him on his way and he'll be with you shortly!"));
    }
    HttpResponse::BadRequest().body("Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?")
}

#[get("/answer3")]
pub async fn answer() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem3::solve()))
}

#[get("/code3")]
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
        r#"
            <p>
                <b>Wrong Answer</b><br/>
                Mhm... That's not going to cut it, I think. Can you find a better solution?
            </p>
        "#
    )
}