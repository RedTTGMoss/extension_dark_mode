use extism_pdk::{FromBytes, ToBytes, Json};
use serde::{Deserialize, Serialize};

#[derive(ToBytes, FromBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct Color {
    pub r: i64,
    pub g: i64,
    pub b: i64,
    pub a: Option<i64>,
}

#[derive(ToBytes, FromBytes, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct TextColors {
    pub foreground: Color,
    pub background: Option<Color>,
}


impl Color {
    pub fn new(r: i64, g: i64, b: i64, a: Option<i64>) -> Self {
        Color { r, g, b, a }
    }
}