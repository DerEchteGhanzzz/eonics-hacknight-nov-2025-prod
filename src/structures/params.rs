use serde::Deserialize;
use crate::structures::{ pizzas::Pizza };

#[derive(Debug, Deserialize)]
pub struct OrderPizza {
    pub pizza: Pizza
}