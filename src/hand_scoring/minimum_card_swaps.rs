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
            return x.saturating_sub(get_highest_value(number_frequencies, 0));
        }
        XOfColor(x) => {
            let color_frequencies = count_colors(hand);
            return x.saturating_sub(get_highest_value(color_frequencies, 0));
        }
        RunOfX(_x) => {}
    }
    todo!()
}

fn get_highest_value<K, V : Ord + Clone>(map: HashMap<K, V>, default: V) -> V {
    map.iter()
        .max_by(|(_, a_value), (_, b_value)| a_value.cmp(&b_value))
        .map(|(_, value)| value.clone())
        .unwrap_or(default)
}

#[cfg(test)]
mod calculate_minimum_card_swaps_test {
    use crate::deck::{Card, CardNumber};
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
                Card::new(Red, 5),
                Card::new(Green, 5),
                Card::new(Purple, 3),
            ],
        );
        assert_eq!(result, 1)
    }

    #[test]
    fn returns_0_on_exact_number_match() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfNumber(2),
            vec![
                Card::new(Red, 5),
                Card::new(Green, 5),
                Card::new(Purple, 3),
            ],
        );
        assert_eq!(result, 0)
    }

    #[test]
    fn returns_0_on_overfulfilled_number_goal() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfNumber(1),
            vec![
                Card::new(Red, 5),
                Card::new(Green, 5),
                Card::new(Purple, 3),
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
                Card::new(Red, 5),
                Card::new(Green, 8),
                Card::new(Red, 3),
            ],
        );
        assert_eq!(result, 2)
    }

    #[test]
    fn returns_0_on_exact_color_match() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfColor(2),
            vec![
                Card::new(Red, 5),
                Card::new(Green, 8),
                Card::new(Red, 3),
            ],
        );
        assert_eq!(result, 0)
    }

    #[test]
    fn returns_0_on_overfulfilled_color_goal() {
        let result = calculate_minimum_card_swaps(
            &PhaseGoal::XOfColor(1),
            vec![
                Card::new(Red, 5),
                Card::new(Green, 8),
                Card::new(Red, 3),
            ],
        );
        assert_eq!(result, 0)
    }
}
