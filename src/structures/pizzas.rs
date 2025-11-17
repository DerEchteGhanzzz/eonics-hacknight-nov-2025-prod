use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Deserialize, EnumIter, Debug, PartialEq, Eq)]
pub enum Pizza {
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

impl Pizza {
    pub fn ingredients(&self) -> Vec<Ingredient>{
        match self {
            Pizza::Margherita        => vec![Ingredient::Sauce, Ingredient::Mozzerella],
            Pizza::Pepperoni         => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Pepperoni],
            Pizza::QuattroFromaggi   => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Gorgonzola, Ingredient::Parmesan, Ingredient::Fontina],
            Pizza::MeatLovers        => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Bacon, Ingredient::Pepperoni],
            Pizza::QuattroStagioni   => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Artichokes, Ingredient::Mushrooms],
            Pizza::Hawaiian          => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Pineapple, Ingredient::Ham],
            Pizza::ProsciuttoEFunghi => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Ham, Ingredient::Mushrooms],
            Pizza::Spinaci           => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Spinach],
            Pizza::TonnoEChipolla    => vec![Ingredient::Sauce, Ingredient::Mozzerella, Ingredient::Tuna, Ingredient::Onion],
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

pub enum Size {
    Small,
    Medium,
    Large,
    American
}

impl Size {
    pub fn size(&self) -> i32 {
        match self {
            Self::Small     => 25,
            Self::Medium    => 29,
            Self::Large     => 35,
            Self::American  => 90
        }
    }

    pub fn from_str(s: &str) -> Option<Size> {
        match s {
            "Small"     => Some(Size::Small),
            "Medium"    => Some(Size::Medium),
            "Large"     => Some(Size::Large),
            "American"  => Some(Size::American),
            _           => Option::None,
        }
    }
}

pub fn available_pizzas() -> Vec<Pizza> {
    Pizza::iter().collect()
}