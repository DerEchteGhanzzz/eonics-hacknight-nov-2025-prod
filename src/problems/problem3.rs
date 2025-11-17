use std::{collections::{HashMap, HashSet}, i32};
use crate::controller::css::CSS;
use crate::problems::parser;

pub fn get_problem() -> String {
    format!(
        r#"
        {}
        <body>
            Okay, this one is a doozy.<br/>
            Of course, we still need to bring the pizzas over to all of our clients.<br/>
            However, to do this as quick as possible, we will need to find some sort of schedule to do that!<br/>
            And, if we have that schedule, we also need to indicate how long it will take as well.<br/>
            Luckily, we can provide you with an <a href="/input3">overview</a> of the travel times between each pair of locations we'll need to visit!<br/>
            Please give the minimum travel time for our courrier to visit every location and come back here again.<br/>
        </body>
        "#,
        CSS
    )
}

pub fn get_input() -> String {
    parser::read_file("./src/input_files/input3.txt")
}

pub fn get_input_lines() -> Vec<String> {
    parser::lines_from_file("./src/input_files/input3.txt")
}

pub fn answer(answer: &str) -> bool {
    let res = i32::from_str_radix(answer, 10);
    if res.is_err() {
        return false;
    }
    let true_answer = solve();
    println!("Problem 3: ours {} == theirs {}", true_answer, answer);
    res.unwrap() == true_answer
}

pub fn solve() -> i32 {
    let mut d_raw = get_input_lines().into_iter();
    let mut d = d_raw.next().unwrap().split(";").skip(1).map(|_| 
        d_raw.next().unwrap().split(";").skip(1).map(|i| i32::from_str_radix(i, 10).unwrap()).collect::<Vec<_>>()
    ).collect::<Vec<Vec<_>>>();
    tsp(vec![0], &d, 0, min_span_tree(&d) * 2)
}

fn min_span_tree(d: &Vec<Vec<i32>>) -> i32 {
    let mut edges = vec![];
    for i in (0..d.len()).rev() {
        for j in (i + 1..d.len()).rev() {
            edges.push((d[i][j], i, j));
        }    
    }

    let mut visited = Vec::new();
    let mut ub = 0;
    edges.sort();
    for (dist, from, to) in edges {
        println!("{dist} {from} {to}");
        if visited.contains(&from) && visited.contains(&to) {
            continue;
        }
        if !visited.contains(&from) {
            visited.push(from);
        }
        if !visited.contains(&to) {
            visited.push(to);
        }
        println!("{ub}");
        ub += dist;
    }
    ub
}

fn tsp(visited: Vec<usize>, d: &Vec<Vec<i32>>, current: i32, ub: i32) -> i32 {
    if visited.len() == d.len() {
        return current + d[*visited.last().unwrap()][*visited.first().unwrap()];
    }
    let mut best = i32::MAX;
    if current > ub {
        return best;
    }
    let mut destinations = d[*visited.last().unwrap()].iter().enumerate().collect::<Vec<_>>();
    destinations.sort_by(|x, y| x.1.cmp(y.1));
    for (destination, dist) in destinations {
        if visited.contains(&destination) {
            continue;
        }
        best = best.min(tsp(
            visited.clone().into_iter().chain(vec![destination]).collect::<Vec<_>>(), 
            d,
            current + dist, 
            ub.min(best))
        );
    }
    best
}

pub fn get_code() -> String {
    String::from(
    r#"<code>
        pub fn solve(d: HashMap<String, HashMap<String, i32>>) {
        |   tsp(vec![d.keys().next().unwrap()], &d, 0, min_span_tree(&d) * 2)
        }

        fn min_span_tree(d: &HashMap<String, HashMap<String, i32>>) -> i32 {
        |    let mut edges = vec![];
        |    for i in (0..d.keys().len()).rev() {
        |        for j in (i + 1..d.keys().len()).rev() {
        |            let from = d.keys().skip(i).next().unwrap();
        |            let to = d.keys().skip(j).next().unwrap();
        |            edges.push((d.get(from).unwrap().get(to).unwrap(), from, to));
        |        }    
        |    }
        |
        |    let mut visited = HashSet::new();
        |    let mut ub = 0;
        |    edges.sort();
        |    for (dist, from, to) in edges {
        |
        |        if visited.contains(from) && visited.contains(to) {
        |            continue;
        |        }
        |        visited.insert(from);
        |        visited.insert(to);
        |        ub += dist;
        |    }
        |    ub
        }

        fn tsp(visited: Vec<&String>, d: &HashMap<String, HashMap<String, i32>>, current: i32, ub: i32) -> i32 {
        |   if visited.len() == d.len() {
        |   |   return current + d[*visited.last().unwrap()][*visited.first().unwrap()];
        |   }
        |   let mut best = i32::MAX;
        |   if current > ub {
        |   |   return best;
        |   }
        |   for destination in d.keys() {
        |   |   if visited.contains(&destination) {
        |   |   |   continue;
        |   |   }
        |   |   let new_visited = visited.iter().chain(vec![&destination]).map(|s| *s).collect::<Vec<&String>>();
        |   |   let dist = d.get(*visited.last().unwrap()).unwrap().get(destination).unwrap();
        |   |   best = best.min(tsp(new_visited, d, current + dist, ub));
        |   }
        |   best
        }
    </code>"#)
}