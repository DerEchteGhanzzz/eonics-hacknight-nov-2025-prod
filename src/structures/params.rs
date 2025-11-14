use serde::Deserialize;
use crate::structures::{ pizzas::PizzaName };

#[derive(Debug, Deserialize)]
pub struct OrderPizza {
    pub name: PizzaName
}

#[derive(Debug, Deserialize)]
pub struct Answer {
    pub answer: String
}