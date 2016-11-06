#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use nom::{alphanumeric, digit, space, eol};

named!(name<&str>,
    map_res!(
        alphanumeric,
        str::from_utf8
    )
);

named!(number<isize>,
    map_res!(
        map_res!(
            recognize!(chain!(
                opt!(char!('-')) ~
                digit,
                || ()
            )),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct Ingredient<'a> {
    name: &'a str,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

named!(pub ingredient<Ingredient>,
    chain!(
        name: name ~ tag!(":") ~ space ~
        tag!("capacity") ~ space ~
        capacity: number ~ tag!(",") ~ space ~
        tag!("durability") ~ space ~
        durability: number ~ tag!(",") ~ space ~
        tag!("flavor") ~ space ~
        flavor: number ~ tag!(",") ~ space ~
        tag!("texture") ~ space ~
        texture: number ~ tag!(",") ~ space ~
        tag!("calories") ~ space ~
        calories: number,
        || Ingredient { name: name, capacity: capacity, durability: durability, flavor: flavor, texture: texture, calories: calories }
    )
);

named!(pub ingredients<Vec<Ingredient> >,
    complete!(
        separated_list!(
            eol,
            ingredient
        )
    )
);

pub struct IngredientCombinations<'a> {
    ingredients: &'a [Ingredient<'a>],
    total: usize,
    amounts: Vec<usize>,
}

impl<'a> IngredientCombinations<'a> {
    fn new(ingredients: &'a [Ingredient], total: usize) -> IngredientCombinations<'a> {
        IngredientCombinations { ingredients: ingredients, total: total, amounts: vec![0; ingredients.len()] }
    }

    fn next(&mut self) -> Option<(usize, Vec<(&Ingredient, usize)>)> {
        let mut i = 0;
        while i < self.amounts.len() {
            if self.amounts[i] < self.total {
                self.amounts[i] += 1;
                i = 0;
            } else {
                self.amounts[i] = 0;
                i += 1;
                continue;
            }
            if self.amounts.iter().sum::<usize>() == self.total {
                let capacity = self.ingredients.iter().zip(self.amounts.iter()).fold(0, |sum, (ing, &amn)| sum + amn as isize * ing.capacity);
                let durability = self.ingredients.iter().zip(self.amounts.iter()).fold(0, |sum, (ing, &amn)| sum + amn as isize * ing.durability);
                let flavor = self.ingredients.iter().zip(self.amounts.iter()).fold(0, |sum, (ing, &amn)| sum + amn as isize * ing.flavor);
                let texture = self.ingredients.iter().zip(self.amounts.iter()).fold(0, |sum, (ing, &amn)| sum + amn as isize * ing.texture);
                let _calories = self.ingredients.iter().zip(self.amounts.iter()).fold(0, |sum, (ing, &amn)| sum + amn as isize * ing.calories);
                let score = if capacity > 0 { capacity as usize } else { 0 } *
                            if durability > 0 { durability as usize } else { 0 } *
                            if flavor > 0 { flavor as usize } else { 0 } *
                            if texture > 0 { texture as usize } else { 0 };
                return Some((score, self.ingredients.iter().zip(self.amounts.clone()).collect()));
            }
        }
        None
    }
}

fn main() {
    let ingredients = ingredients(include_str!("day15.txt").as_bytes()).unwrap().1;
    let mut combination = IngredientCombinations::new(&ingredients, 100);
    let mut max_score = 0;
    while let Some((score, _)) = combination.next() {
        if score > max_score { max_score = score }
    }
    println!("Max cookie score: {}", max_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(ingredient(b"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8").unwrap(),
            (&b""[..], Ingredient { name: "Butterscotch", capacity: -1, durability: -2, flavor: 6, texture: 3, calories: 8 }));
        assert_eq!(ingredients(include_str!("day15.txt").as_bytes()).unwrap().1.len(), 4);
    }

    #[test]
    fn permutation() {
        let ingredients = ingredients(b"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\nCinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3").unwrap().1;
        let butterscotch = &ingredients[0];
        let cinnamon = &ingredients[1];
        let mut combination = IngredientCombinations::new(&ingredients, 3);
        assert_eq!(combination.next(), Some((0, vec![(butterscotch, 3), (cinnamon, 0)])));
        assert_eq!(combination.next(), Some((0, vec![(butterscotch, 2), (cinnamon, 1)])));
        assert_eq!(combination.next(), Some((24, vec![(butterscotch, 1), (cinnamon, 2)])));
        assert_eq!(combination.next(), Some((0, vec![(butterscotch, 0), (cinnamon, 3)])));
        assert_eq!(combination.next(), None);
    }
}
