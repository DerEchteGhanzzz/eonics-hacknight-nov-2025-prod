use crate::structures::pizzas::Size;
use crate::problems::parser;
use crate::controller::css::CSS;

pub fn get_problem() -> String {
    format!(
        r#"
        {}
        <body>
            <h2>
                Look, there's still a long long queue of people before I can get to your order.<br/>
                You know what? If I could bake all pizzas at the same time, I could then immediately do yours too!
                To make this much pizzas, we will need an enormous oven!<br/><br/>

                But, for that you can use our trademarked PiZZA formula:<br/>
                If you consider the Pizza Constant Pi (Pi = 3), and you have a pizza of diameter Z, its area A will be:<br/>
                Pi * Z * Z = A<br/>
                The sizes are Small z=25, Medium z=29, Large z=35, American z=90<br/>
                Please calculate the area we'd need to bake all pizzas at once by summing up the areas of all pizzas.
                The pizzas we still need to bake can be found in the <a href="/input1">order sheet</a><br/>
            </h2>
        </body>
        "#,
        CSS
    )
}

pub fn get_input() -> String {
    parser::read_file("./src/input_files/input1.txt")
}

pub fn get_input_lines() -> Vec<String> {
    parser::lines_from_file("./src/input_files/input1.txt")
}

pub fn answer(answer: &str) -> bool {
    answer == format!("{}", solve())
}

pub fn solve() -> i32 {
    let pi = 3;
    get_input_lines()
        .into_iter()
        .map(|s| Size::from_str(&s).unwrap().size())
        .map(|z| pi * z * z).sum()
}

pub fn get_code() -> String {
    String::from(
        r#"
        <code>
        |fn size_to_int(size: &String) {
        |   match size {
        |       "Small"    => 25,
        |       "Medium"   => 29,
        |       "Large"    => 35,
        |       "American" => 90,
        |       _          => panic!("Not a size!")
        |   }
        |}
        |
        |fn solve(sizes: Vec<String>) {
        |    let pi = 3;
        |    sizes().into_iter()
        |        .map(|s| size_to_int(&s).unwrap().size())
        |        .map(|z| pi * z * z).sum()
        |}
        </code>
        "#
    )
}
