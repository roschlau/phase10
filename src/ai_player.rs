use crate::visible_game_state::VisibleGameState;
use crate::deck::Card;
use crate::phases::PhaseGoal;

struct AiPlayer {}

impl AiPlayer {
    fn player_draw(&self, _game_state: VisibleGameState) -> DrawSource {
        todo!()
    }

    fn player_come_out(&self, _game_state: VisibleGameState) -> Option<Vec<(PhaseGoal, Vec<Card>)>> {
        todo!()
    }

    fn player_add_to_phases(&self, _game_state: VisibleGameState) -> Option<Vec<(PhaseGoal, Vec<Card>)>> {
        todo!()
    }

    fn player_discard(&self, _game_state: VisibleGameState) -> Card {
        todo!()
    }
}

enum DrawSource {
    Discard,
    Random,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
