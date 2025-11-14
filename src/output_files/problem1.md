So, yea, thank you for helping us... 
The problem is that we've got this enormous storage, 
but we don't know how many pizzas we'll be able to make.
Can you figure out how many of each pizza type we will be able to make? 
For each pizza in our menu (/menu), you can get a list of its ingredients via /ingredients. 
Our storage can be accessed via /storage. It will return a list of all our ingredients.

So, in Rust a Vec has a function called `iter()`. It turns the list into an iterator (like `Stream` in Java).
If you've got a Vec of pizzas, you will be able to call `.iter().map()` on it.
Within the `map()` function, you can place a helper function to help you count, like this:
`pizza_list.iter().map(|p| count_pizzas(p))`. `|p| count_pizzas(p)` is a lambda function.
This way you can easily turn your list of pizzas into a list of integers, without a bulky for loop or side-effects.
However, before sending your result off, don't forget to collect your iterator again. You want to turn the resulting iterator into a list again.
Do this with `.collect::<Vec<_>>()`. The collect function turns an iterator into another data structure.
The `::<Vec<_>>` part, called the TURBO FISH tells the compiler you want to collect the iterator back into an array.

Send your result as a formatted list like: `[0, 0, 0, 0, 0]` (use: `format!(\"{:?}\", result)` to turn your list into a string) to /answer1. 
Each index in the list corresponds with each item on our menu.