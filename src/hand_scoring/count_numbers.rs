use std::collections::HashMap;
use crate::deck::{Card, CardNumber};

pub fn count_numbers(hand: Vec<Card>) -> HashMap<CardNumber, u8> {
    let mut number_frequencies: HashMap<CardNumber, u8> = HashMap::new();
    for card in hand {
        match card {
            Card::Joker => {}
            Card::Skip => {}
            Card::Number(_, number) => {
                let count = number_frequencies.entry(number).or_insert(0);
                *count += 1;
            }
        }
    }
    number_frequencies
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::deck::{Card, CardNumber};
    use crate::hand_scoring::count_numbers::count_numbers;
    use crate::deck::Color::{Green, Purple, Red};

    #[test]
    fn count_numbers_returns_0_on_empty_hand() {
        let result = count_numbers(vec![]);
        assert_eq!(result, HashMap::from([]));
    }

    #[test]
    fn count_numbers_ignores_special_cards() {
        let result = count_numbers(vec![
            Card::Joker,
            Card::Skip,
            Card::Joker,
        ]);
        assert_eq!(result, HashMap::from([]));
    }

    #[test]
    fn count_numbers_counts_numbers_disregarding_color() {
        let result = count_numbers(vec![
            Card::new(Red, 1),
            Card::new(Green, 1),
            Card::new(Red, 3),
        ]);
        assert_eq!(result, HashMap::from([
            (CardNumber(1), 2),
            (CardNumber(3), 1),
        ]));
    }

    #[test]
    fn count_numbers_full_test() {
        let result = count_numbers(vec![
            Card::new(Red, 1),
            Card::new(Green, 1),
            Card::Skip,
            Card::new(Purple, 3),
            Card::Joker,
        ]);
        assert_eq!(result, HashMap::from([
            (CardNumber(1), 2),
            (CardNumber(3), 1),
        ]));
    }
}
