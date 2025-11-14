use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::css::CSS, structures::{params::{Answer, OrderPizza}, pizzas::available_pizzas}};
use crate::problems::{problem2, problem1, problem3};

fn order_error() -> String {
    String::from(
        r#"
            <p>
                api usage:<br/>
                <b>GET</b><br/>
                params:<br/>
                <b>pizza</b>: Name of pizza<br/>
                check <a href="/menu">/menu</a> for available pizzas.
            </p>
        "#
    )
}

fn wrong_answer() -> String {
    String::from(
        r#"
            <p>
                Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?
            </p>
        "#
    )
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        format!(r#"
        {}
        <body>
            <div>
                <h1>
                    Welcome to the Rusty Krab, we're <a href="./open">open</a>, may I take your order please?
                </h1>
                <img src="https://media1.tenor.com/m/-lohISybXxoAAAAd/spongebob-the-krusty-krab-pizza.gif"/>
                <img src="https://rustacean.net/assets/rustacean-flat-happy.png"/>
                <img src="https://media1.tenor.com/m/Vf6z7MDoM4IAAAAd/lolol-is-this-krusty-krab.gif"/></br>
                <h2>View our <a href="./menu">menu</a>.</h2>
            </div>
        </body>
        "#
    , CSS))
}

#[get("/open")]
pub async fn open() -> impl Responder {
    HttpResponse::Ok().body(
        format!(r#"
        {}
        <body>
            <h2>
                We're open! Come in! Check our menu via 
                <a href="./menu">/menu</a>, and check 
                <a href="./ingredients">ingredients</a> for what each pizza contains.
                To order a pizza, you can call <a href="./order-pizza">/order-pizza</a> with your pizza of choice from the menu.
            </h2>
        </body>
        "#
    , CSS))
}

#[get("/menu")]
pub async fn menu() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", available_pizzas()))
}

#[get("/order-pizza")]
pub async fn order_pizza(req: HttpRequest) -> impl Responder {
    let request = web::Query::<OrderPizza>::from_query(req.query_string());
    if request.is_err() {
        return HttpResponse::BadRequest().body(format!("{}{}", CSS, order_error()));
    }
    let order = &request.unwrap().pizza;
    HttpResponse::Ok().body(format!(r#"
        {}
        <body>
            <h2>
                Yea... About that... You see, there are currently a lot of events taking place... 
                So, it might take a while for you to get your {:?}... 
                Actually, you might be able to help me!
                If you solve my 3 problems, I'll give you your pizza for free!<br/>
                You can find the first problem via <a href="/problem1">/problem1<a>
            </h2>
        </body>
        "#, CSS, order))
}

#[get("/ingredients")]
pub async fn get_ingredients(req: HttpRequest) -> impl Responder {
    let request = web::Query::<OrderPizza>::from_query(req.query_string());
    if request.is_err() {
        return HttpResponse::BadRequest().body(format!("{}{}", CSS, order_error()));
    }
    let order = &request.unwrap().pizza;
    HttpResponse::Ok().body(format!("{:?}", order.ingredients()))
}

#[get("/problem1")]
pub async fn get_problem1() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::get_problem()))
}

#[get("/input1")]
pub async fn get_input1() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::get_input()))
}

#[get("/solve1")]
pub async fn solve1(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem1::answer(&request.unwrap().answer) {
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
    HttpResponse::BadRequest().body(format!("{}{}", CSS, wrong_answer()))
}

#[get("/answer1")]
pub async fn answer1() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::solve()))
}

#[get("/code1")]
pub async fn code1() -> impl Responder {
    HttpResponse::Ok().body(format!("
        {}
        <body>
        {}
        </body>
        ", CSS, problem1::get_code().replace("\n", "<br/>"))
    )
}

#[get("/problem2")]
pub async fn get_problem2() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_problem()))
}

#[get("/input2")]
pub async fn get_input2() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_input()))
}

#[get("/solve2")]
pub async fn solve2(req: HttpRequest) -> impl Responder {
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
pub async fn answer2() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem2::solve()))
}

#[get("/code2")]
pub async fn code2() -> impl Responder {
    HttpResponse::Ok().body(format!("
        {}
        <body>
        {}
        </body>
        ", CSS, problem2::get_code().replace("\n", "<br/>"))
    )
}

#[get("/problem3")]
pub async fn get_problem3() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_problem()))
}

#[get("/input3")]
pub async fn get_input3() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_input()))
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

#[get("/code3")]
pub async fn code3() -> impl Responder {
    HttpResponse::Ok().body(format!("
        {}
        <body>
        {}
        </body>
        ", CSS, problem3::get_code().replace("\n", "<br/>"))
    )
}