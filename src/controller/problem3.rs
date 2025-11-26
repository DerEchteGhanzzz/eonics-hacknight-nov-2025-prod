use crate::problems::problem3;
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use serde::Deserialize;
use crate::{controller::{css::CSS, receiver}};

#[get("/problem3/definition")]
pub async fn get_problem() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_problem()))
}


#[get("/problem3/locations")]
pub async fn get_locations() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_locations()))
}

#[derive(Deserialize, Debug)]
struct FromToParams {
    from: String,
    to: String,
}

#[get("/problem3/from-to")]
pub async fn get_from_to(req: HttpRequest) -> impl Responder {
    let request = web::Query::<FromToParams>::from_query(req.query_string());
    if request.is_err() {
        return HttpResponse::BadRequest().body(format!(r#"{}{}"#, CSS, from_to_error()));
    }
    let from_to = &request.unwrap();
    if let Some(dist) = problem3::get_from_to(&from_to.from, &from_to.to) {
        return HttpResponse::Ok().body(format!("{:?}", dist));
    }
    return HttpResponse::BadRequest().body(format!("{from_to:?} is not a valid edge"));
}

fn from_to_error() -> String {
    String::from(
        r#"
            <p>
                api usage:<br/>
                <b>GET</b><br/>
                params:<br/>
                <b>from</b>: origin<br/>
                <b>to</b>: destination<br/>
                check <a href="/problem3/locations">this</a> for available locations.
            </p>
        "#
    )
}

#[post("/problem3/solve")]
pub async fn solve(payload: web::Payload) -> impl Responder {
    let body = receiver::handle_post::<i32>(payload).await;
    if body.is_err() {
        return HttpResponse::NotAcceptable().body(format!("{:?}", body));
    }
    println!("{:?}", body);
    let ans = &body.unwrap().to_string();
    if problem3::answer(ans) {
        return HttpResponse::Ok().body(format!(
            "{ans} is correct!"
        ));
    }
    HttpResponse::BadRequest().body(format!("{} is the wrong Answer, please try again", ans))
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