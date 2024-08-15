use crate::deck::Card;
use crate::hand_scoring::count_numbers;
use crate::phases::PhaseGoal;
use crate::phases::PhaseGoal::{RunOfX, XOfColor, XOfNumber};

fn calculate_minimum_card_swaps(goal: &PhaseGoal, hand: Vec<Card>) -> Option<u8> {
    match goal {
        XOfNumber(x) => {
            let number_frequencies = count_numbers::count_numbers(hand);
            return number_frequencies
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|max_entry| max_entry.1.clone())
                .map(|max_number_count| x - &max_number_count);
        }
        XOfColor(_x) => {}
        RunOfX(_x) => {}
    }
    todo!()
}
