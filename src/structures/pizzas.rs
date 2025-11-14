use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Deserialize, EnumIter, Debug, PartialEq, Eq)]
pub enum PizzaName {
    Margherita,
    Pepperoni,
    MeatLovers,
    QuattroFromaggi,
    QuattroStagioni,
    Hawaiian,
    ProsciuttoEFunghi,
    Spinaci,
    TonnoEChipolla
}

impl PizzaName {
    pub fn ingredients(&self) -> Vec<Ingredient>{
        match self {
            PizzaName::Margherita        => vec![Ingredient::Sauce, Ingredient::Mozzerella],
            PizzaName::Pepperoni         => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Pepperoni],
            PizzaName::QuattroFromaggi   => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Gorgonzola, Ingredient::Parmesan, Ingredient::Fontina],
            PizzaName::MeatLovers        => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Bacon, Ingredient::Pepperoni],
            PizzaName::QuattroStagioni   => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Artichokes, Ingredient::Mushrooms, Ingredient::Mushrooms],
            PizzaName::Hawaiian          => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Pineapple, Ingredient::Ham],
            PizzaName::ProsciuttoEFunghi => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Ham, Ingredient::Mushrooms],
            PizzaName::Spinaci           => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Spinach],
            PizzaName::TonnoEChipolla    => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Tuna, Ingredient::Onion],
        }
    }
}

#[derive(Deserialize, EnumIter, Debug, PartialEq, Eq)]
pub enum Ingredient {
    Sauce,
    Mozzerella,
    Pepperoni,
    Gorgonzola, 
    Fontina,
    Parmesan,
    Artichokes,
    Mushrooms,
    Ham,
    Olives,
    Pineapple,
    Spinach,
    Tuna,
    Onion,
    Bacon
}

impl Ingredient {

    pub fn from_str(s: &str) -> Option<Ingredient> {
        match s {
            "Sauce"         => Option::Some(Ingredient::Sauce),
            "Mozzerella"    => Option::Some(Ingredient::Mozzerella),
            "Pepperoni"     => Option::Some(Ingredient::Pepperoni),
            "Gorgonzola"    => Option::Some(Ingredient::Gorgonzola,),
            "Fontina"       => Option::Some(Ingredient::Fontina),
            "Parmesan"      => Option::Some(Ingredient::Parmesan),
            "Artichokes"    => Option::Some(Ingredient::Artichokes),
            "Mushrooms"     => Option::Some(Ingredient::Mushrooms),
            "Ham"           => Option::Some(Ingredient::Ham),
            "Olives"        => Option::Some(Ingredient::Olives),
            "Pineapple"     => Option::Some(Ingredient::Pineapple),
            "Spinach"       => Option::Some(Ingredient::Spinach),
            "Tuna"          => Option::Some(Ingredient::Tuna),
            "Onion"         => Option::Some(Ingredient::Onion),
            "Bacon"         => Option::Some(Ingredient::Bacon),
            _               => Option::None,
        }
    }
}

pub fn available_pizzas() -> Vec<PizzaName> {
    PizzaName::iter().collect()
}

pub fn all_ingredients() -> Vec<Ingredient> {
    Ingredient::iter().collect()
}