use std::collections::HashMap;
use crate::deck::Card;
use crate::hand_scoring::count_colors::count_colors;
use crate::hand_scoring::count_numbers::count_numbers;
use crate::phases::PhaseGoal;
use crate::phases::PhaseGoal::{RunOfX, XOfColor, XOfNumber};

fn calculate_minimum_card_swaps(goal: &PhaseGoal, hand: Vec<Card>) -> u8 {
    match goal {
        XOfNumber(x) => {
            let number_frequencies = count_numbers(hand);
            return x - get_highest_entry(number_frequencies, 0);
        }
        XOfColor(x) => {
            let color_frequencies = count_colors(hand);
            return x - get_highest_entry(color_frequencies, 0);
        }
        RunOfX(_x) => {}
    }
    todo!()
}

fn get_highest_entry<K>(map: HashMap<K, u8>, default: u8) -> u8 {
    map.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|max_entry| max_entry.1.clone())
        .unwrap_or(default)
}

#[cfg(test)]
mod calculate_minimum_card_swaps_test {
    use crate::deck::Card;
    use crate::deck::Color::{Green, Purple, Red};
    use crate::hand_scoring::minimum_card_swaps::calculate_minimum_card_swaps;
    use crate::phases::PhaseGoal;

    #[test]
    fn returns_number_count_goal_count_on_empty_hand() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfNumber(5),
            vec![],
        );
        assert_eq!(result, 5)
    }

    #[test]
    fn returns_remaining_number_swaps() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfNumber(3),
            vec![
                Card::Number { number: 5, color: Red },
                Card::Number { number: 5, color: Green },
                Card::Number { number: 3, color: Purple },
            ],
        );
        assert_eq!(result, 1)
    }

    #[test]
    fn returns_0_on_exact_number_match() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfNumber(2),
            vec![
                Card::Number { number: 5, color: Red },
                Card::Number { number: 5, color: Green },
                Card::Number { number: 3, color: Purple },
            ],
        );
        assert_eq!(result, 0)
    }

    #[test]
    fn returns_color_count_goal_count_on_empty_hand() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfColor(5),
            vec![],
        );
        assert_eq!(result, 5)
    }

    #[test]
    fn returns_remaining_color_swaps() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfColor(4),
            vec![
                Card::Number { number: 5, color: Red },
                Card::Number { number: 8, color: Green },
                Card::Number { number: 3, color: Red },
            ],
        );
        assert_eq!(result, 2)
    }

    #[test]
    fn returns_0_on_exact_color_match() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfColor(2),
            vec![
                Card::Number { number: 5, color: Red },
                Card::Number { number: 8, color: Green },
                Card::Number { number: 3, color: Red },
            ],
        );
        assert_eq!(result, 0)
    }
}
