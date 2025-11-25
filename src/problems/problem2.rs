use crate::structures::pizzas::{Ingredient, Pizza, available_pizzas};
use crate::problems::parser;
use crate::controller::css::CSS;

pub fn get_problem() -> String {
    format!(r#"
        {}
        <body>
            <h2>
            Again, thank you for helping us. 
            The problem is that we've got this enormous storage, 
            but we don't know how many pizzas we'll be able to make.
            For each pizza in our <a href="//rustykrab.nl/menu">menu</a>, you can get a list of its ingredients by calling /ingredients?pizza=pizzaName. 
            Our storage can be accessed via <a href="/problem2/input">here</a>. It will return a list of all our ingredients.<br/>
            Calculate for each pizza type <u>separately</u> how many can be made with the ingredients in the storage.<br/><br/>

            Send your result as a formatted vector like: `[0, 0, 0, ..]` (use: `format!(\"{{:?}}\", result)` to turn your vector into a string) to /problem2/answer. 
            Each index in the list corresponds with each item on our menu.
            </h2>
            If you're done, you can go back to <a href="/order-pizza">order a pizza</a>
        </body>
    "#, CSS)
}

pub fn get_input() -> String {
    parser::read_file("./src/input_files/input2.txt")
}

pub fn get_input_lines() -> Vec<String> {
    parser::lines_from_file("./src/input_files/input2.txt")
}

pub fn answer(answer: &str) -> bool {
    let true_answer = parser::read_file("./src/output_files/answer2.txt");
    println!("Problem 2: ours {true_answer}, theirs: {answer}");
    true_answer == answer
}

pub fn solve() -> Vec<i32> {
    let storage = get_input_lines().iter().map(|i_str| Ingredient::from_str(i_str).unwrap()).collect::<Vec<Ingredient>>();
    let menu = available_pizzas();
    menu.iter().map(|pizza| count_occurrances(pizza, &storage)).collect::<Vec<_>>()
}

fn count_occurrances(pizza: &Pizza, storage: &Vec<Ingredient>) -> i32 {
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

pub fn get_code() -> String {
    String::from(
        r#"
        <code>
        |pub fn solve(menu: Vec<String>, ingredients: Vec<Vec<String>>, storage: Vec<String>) -> Vec<i32> {
        |    menu.iter().enumerate().map(|idx, pizza| count_occurrances(&pizza, &ingredients[idx], &storage))
        |        .collect::<Vec<_>>()
        |}
        |
        |fn count_occurrances(pizza: &String, ingredients: &Vec<String>, storage: &Vec<String>) -> i32 {
        |    let mut answer = ingredients.iter().map(|_| 0).collect::<Vec<_>>();
        |    for ingredient in storage {
        |        let idx = ingredients.iter().position(|x| x == ingredient);
        |        if idx.is_none() {
        |            continue;
        |        }
        |        answer[idx.unwrap()] += 1;
        |    }
        |    *answer.iter().min().unwrap()
        |}
        </code>
        "#
    )
}