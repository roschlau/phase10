use rand::thread_rng;
use rand::prelude::SliceRandom;
use crate::deck::Card::{Joker, Number, Skip};
use crate::deck::Color::{Green, Purple, Red, Yellow};

fn create_shuffled_deck() -> Vec<Card> {
    let mut result = vec![];
    result.append(&mut vec![Joker; 8]);
    result.append(&mut vec![Skip; 4]);
    for color in [Red, Green, Yellow, Purple] {
        for number in 1..=12 {
            result.append(&mut vec![Number(color, number); 2])
        }
    }
    result.shuffle(&mut thread_rng());
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Card {
    Joker,
    Skip,
    Number(Color, u8)
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Purple,
}
