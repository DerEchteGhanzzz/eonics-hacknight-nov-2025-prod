fn tuple() {
    let tuple = ('x', 69);
    tuple.0; // 'a'
    tuple.1; // 69

    // destructuring a tuple
    let (a, b) = tuple;
    a; // 'x'
    b; // 69
}

fn hello_world<'a>() -> &'a str {
    return "hello world";
}

// fn steal_variable() {
//     let x = String::from("Welcome message");
//     some_function(x);
//     println!("{}", x);
// }

// fn some_function(welcome: String) {
//     println!("{}", welcome);
// }

fn borrow_variable() {
    let x = String::from("Welcome message");
    some_function(&x);
    println!("{}", x);
}

fn some_function(welcome: &str) {
    println!("{}", welcome);
}

fn append(list: &mut Vec<i32>) {
    list.push(0);
}

fn print_list(list: &Vec<i32>) {
    println!("{:?}", list);
}