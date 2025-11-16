use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::{controller::css::CSS, structures::{params::{Answer, OrderPizza}, pizzas::available_pizzas}};

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

