use crate::deck::Card;
use std::collections::HashMap;

fn runs_by_length(hand: Vec<Card>) -> HashMap<u8, Vec<Card>> {
    let mut number_cards = hand.into_iter()
        .filter_map(|card| match card {
            Card::Number(_, number) => Some((number, card)),
            _ => None,
        })
        .collect::<Vec<_>>();
    number_cards.sort_by(|(a, _), (b, _)| a.cmp(b));
    if number_cards.is_empty() {
        HashMap::new()
    } else {
        HashMap::from([(1, vec![number_cards[0].1.clone()])])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::Card::{Joker, Skip};
    use crate::deck::Color::Yellow;

    #[test]
    fn returns_empty_map_on_empty_hand() {
        let result = runs_by_length(vec![]);
        assert_eq!(HashMap::new(), result)
    }

    #[test]
    fn returns_empty_map_on_hand_with_only_special_cards() {
        let result = runs_by_length(vec![
            Joker, Joker, Skip
        ]);
        assert_eq!(HashMap::new(), result)
    }

    #[test]
    fn returns_singleton_run_on_hand_with_one_card() {
        let card = Card::Number(Yellow, 5);
        let result = runs_by_length(vec![card]);
        assert_eq!(
            HashMap::from([(1, vec![Card::Number(Yellow, 5)])]),
            result,
        )
    }
}
