use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::css::CSS, structures::{params::{Answer, OrderPizza}, pizzas::available_pizzas}};
use crate::problems::{problem2, problem1, problem3};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        format!(r#"
        {}
        <body>
            <div>
                <h1>
                    The Rusty Krab pizza, is the pizza, YEAAAAH, for you and MEEEEEEEE
                </h1>
                Welcome to the Rusty Krab, we're <a href="./open">open</a>, may I take your order please?<br/>
                <img src="https://media1.tenor.com/m/-lohISybXxoAAAAd/spongebob-the-krusty-krab-pizza.gif"/>
                <img src="https://rustacean.net/assets/rustacean-flat-happy.png"/>
                <img src="https://media1.tenor.com/m/Vf6z7MDoM4IAAAAd/lolol-is-this-krusty-krab.gif"/></br>
                View our <a href="./menu">menu</a>.</br>
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
            <div>
                We're open! Come in! Check our menu via 
                <a href="./menu">/menu</a>, and check 
                <a href="./ingredients">ingredients</a> for what each pizza contains.
                To order a pizza, you can call <a href="./order-pizza">/order-pizza</a> with your pizza of choice from the menu.
            </div>
        </body>
        "#
    , CSS))
}

#[get("/menu")]
pub async fn menu() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", available_pizzas()))
}

#[get("/order-pizza")]
pub async fn order_pizza() -> impl Responder {
    HttpResponse::Ok().body("Yea... About that... You see, there are a lot of events open currently.")
}

#[get("/ingredients")]
pub async fn get_ingredients(req: HttpRequest) -> impl Responder {
    let request = web::Query::<OrderPizza>::from_query(req.query_string());
    if request.is_err() {
        return HttpResponse::BadRequest().body("params usage:\n\tname: Name of pizza; check /menu for our the menu.");
    }
    let order = &request.unwrap().pizza;
    HttpResponse::Ok().body(format!("{:?}", order.ingredients()))
}

#[get("/problem1")]
pub async fn get_problem1() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem2::get_problem()))
}

#[get("/solve1")]
pub async fn solve1(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem2::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!("Oh, very good. Thank you for checking! That streamlines everything nicely"));
    }
    HttpResponse::BadRequest().body("Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?")
}

#[get("/answer1")]
pub async fn answer1() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem2::solve()))
}

#[get("/problem2")]
pub async fn get_problem2() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem1::get_problem()))
}

#[get("/solve2")]
pub async fn solve2(req: HttpRequest) -> impl Responder {
    let request = web::Query::<Answer>::from_query(req.query_string());
    if request.is_ok() && problem1::answer(&request.unwrap().answer) {
        return HttpResponse::Ok().body(format!("Oh, very good. Thank you for checking! That streamlines everything nicely"));
    }
    HttpResponse::BadRequest().body("Okay... nicely done... erm... Only, I don't that's right, could you try looking at it again?")
}

#[get("/answer2")]
pub async fn answer2() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", problem1::solve()))
}

#[get("/problem3")]
pub async fn get_problem3() -> impl Responder {
    HttpResponse::Ok().body(format!("{}", problem3::get_problem()))
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