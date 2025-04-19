// use std::{
//     fmt::Display,
//     ops::{Add, Mul},
// };

// #[derive(Debug)]
// pub struct Player {
//     pub name: String,
//     pub strength: f64,
//     pub score: i32,
//     pub money: i32,
//     pub weapons: Vec<String>,
// }

// pub struct Fruit {
//     pub weight_in_kg: f64,
// }

// pub struct Meat {
//     pub weight_in_kg: f64,
//     pub fat_content: f64,
// }

// impl Player {
//     pub fn eat<T: Food>(&mut self, food: T) {
//         self.strength += food.gives();
//     }
// }

// impl Display for Player {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f, 
//             "{}\nStrength: {}, Score: {}, Money: {}\nWeapons: {:?}", 
//             self.name, self.strength, self.score, self.money, self.weapons
//         )
//     }
// }

// pub trait Food {
//     fn gives(&self) -> f64;
// }

// impl Food for Fruit {
//     fn gives(&self) -> f64 {
//         self.weight_in_kg.mul(4.0)
//     }
// }

// impl Food for Meat {
//     fn gives(&self) -> f64 {
//         (self.weight_in_kg - self.fat_content)
//             .mul(4.0)
//             .add(self.fat_content.mul(9.0))
//     }
// }