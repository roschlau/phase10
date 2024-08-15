use crate::deck::Card;

pub struct VisibleGameState {
    phase: u8,
    hand: Vec<Card>,
    discard_top: Card,
    players: Vec<OtherPlayerVisibleState>
}

struct OtherPlayerVisibleState {
    phase: u8,
    hand_count: u8,
    is_out: bool,
}
