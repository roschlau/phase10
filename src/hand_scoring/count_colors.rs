use std::collections::HashMap;
use crate::deck::{Card, Color};

pub fn count_colors(hand: Vec<Card>) -> HashMap<Color, u8> {
    let mut number_frequencies: HashMap<Color, u8> = HashMap::new();
    for card in hand {
        match card {
            Card::Joker => {}
            Card::Skip => {}
            Card::Number { number: _number, color } => {
                let count = number_frequencies.entry(color).or_insert(0);
                *count += 1;
            }
        }
    }
    number_frequencies
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::deck::Card;
    use crate::hand_scoring::count_colors::count_colors;
    use crate::deck::Color::{Green, Purple, Red};

    #[test]
    fn count_colors_returns_0_on_empty_hand() {
        let result = count_colors(vec![]);
        assert_eq!(result, HashMap::from([]));
    }

    #[test]
    fn count_colors_ignores_special_cards() {
        let result = count_colors(vec![
            Card::Joker,
            Card::Skip,
            Card::Joker,
        ]);
        assert_eq!(result, HashMap::from([]));
    }

    #[test]
    fn count_colors_counts_colors_disregarding_number() {
        let result = count_colors(vec![
            Card::Number { number: 1, color: Red },
            Card::Number { number: 1, color: Green },
            Card::Number { number: 3, color: Red },
        ]);
        assert_eq!(result, HashMap::from([
            (Red, 2),
            (Green, 1),
        ]));
    }

    #[test]
    fn count_colors_full_test() {
        let result = count_colors(vec![
            Card::Number { number: 1, color: Red },
            Card::Number { number: 1, color: Green },
            Card::Number { number: 6, color: Red },
            Card::Skip,
            Card::Number { number: 3, color: Purple },
            Card::Joker,
        ]);
        assert_eq!(result, HashMap::from([
            (Red, 2),
            (Green, 1),
            (Purple, 1),
        ]));
    }
}
