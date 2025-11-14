use crate::structures::pizzas::{Ingredient, PizzaName, available_pizzas};
use serde_json;
use crate::problems::parser;
use crate::controller::css::CSS;

pub fn get_problem() -> String {
    format!(
        r#"
        {}
        <body>
            There is still a long long queue of people before I can get to your order.
            You know what? If I could bake all pizzas at the same time, I could then immediately do yours too!
            To make this much pizzas, we will need an enormous oven!

            Here, you can use our trademarked PiZZA formula:
            If you consider the pizza constant Pi (Pi = 3), and you have a pizza of diameter Z, its area will be:
            Pi * Z * Z = A
            you can find the order sheet in /orders
            The sizes are Small z=25, Medium z=29, Large z=35, American z=90 
        </body>
        "#,
        CSS
    )
}

pub fn get_input() -> Vec<String> {
    parser::lines_from_file("./src/input_files/input1.txt")
}

pub fn answer(answer: &str) -> bool {
    todo!()
}

pub fn solve() -> () {
    todo!();
}

