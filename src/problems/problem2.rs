use crate::structures::pizzas::{Ingredient, PizzaName, available_pizzas};
use serde_json;
use crate::problems::parser;

pub fn get_problem() -> String {
    parser::read_file("./src/output_files/problem2.md")
}

pub fn get_input() -> Vec<String> {
    parser::lines_from_file("./src/input_files/input2.txt")
}

pub fn answer(answer: &str) -> bool {
    todo!()
}

pub fn solve() -> () {
    todo!();
}

// pi z z = A
// Okay, this is great. But... I've got another problem.
// There is still a long long queue of people before I can get to your order.
// You know what? If I could bake all pizzas at the same time, I could then immediately do yours too!
// To make this much pizzas, we will need an enormous oven!
//
// Here, you can use our trademarked PiZZA formula:
// If you consider the pizza constant Pi (Pi = 3), and you have a pizza of diameter Z, its area will be:
// Pi * Z * Z = A
// you can find the diameters of our sizes in the handle /sizes
// and the order sheet in /orders