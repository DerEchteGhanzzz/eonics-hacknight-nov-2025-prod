use crate::structures::pizzas::{Ingredient, PizzaName, available_pizzas};
use serde_json;
use crate::problems::parser;

pub fn get_problem() -> String {
    parser::read_file("./src/output_files/problem1.md")
}

pub fn get_input() -> Vec<String> {
    parser::lines_from_file("./src/input_files/input1.txt")
}

pub fn answer(answer: &str) -> bool {
    println!("Sent in: {}", answer);
    let real_ans = solve();
    println!("Actual:  {:?}", real_ans);
    format!["{:?}", real_ans] == answer
}

pub fn solve() -> Vec<i32> {
    let storage = get_input().iter().map(|i_str| Ingredient::from_str(i_str).unwrap()).collect::<Vec<Ingredient>>();
    let menu = available_pizzas();
    menu.iter().map(|pizza| count_occurrances(pizza, &storage)).collect::<Vec<_>>()
}

fn count_occurrances(pizza: &PizzaName, storage: &Vec<Ingredient>) -> i32 {
    let ingredients = pizza.ingredients();
    let mut answer = ingredients.iter().map(|_| 0).collect::<Vec<_>>();
    for ingredient in storage {
        let idx = ingredients.iter().position(|x| x == ingredient);
        if idx.is_none() {
            continue;
        }
        answer[idx.unwrap()] += 1;
    }
    *answer.iter().min().unwrap()
}