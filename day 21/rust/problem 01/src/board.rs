use log::info;

use crate::{
    die::{Det100Die, Die},
    player::Player,
};

const BOARD_SIZE: usize = 10;
const WINNING_SCORE: usize = 1000;
const DICE_THROW_TIMES: usize = 3;

#[derive(Clone, Debug)]
struct DiracDiceBoard {
    players: &'static Vec<Player>,
    next_player_index: usize,
    next_player: &'static Player,
    positions: usize,
    die: Det100Die,
    player_won_index: Option<usize>,
}

impl DiracDiceBoard {
    fn new(players: &'static Vec<Player>) -> Self {
        let positions = BOARD_SIZE;
        let die = Det100Die::new();
        let next_player_index = 0;
        let next_player = players.get(next_player_index).unwrap();
        let player_won_index = None;

        Self {
            players,
            next_player_index,
            next_player,
            positions,
            die,
            player_won_index,
        }
    }

    fn next_round(&mut self) {
        let mut player = &self.next_player();

        (0..DICE_THROW_TIMES).for_each(|_| {
            let result = &self.die.roll();
            self.advance_player(*player, *result);
        });
        player.recalculate_score();

        info!(
            "Player {} moves to space {} for a total score of {}.",
            player.number(),
            player.current_position(),
            player.score()
        );

        if Self::is_winner(**player) {
            self.player_won_index = Some(self.next_player_index);
            return;
        }

        self.next_player_index = (self.next_player_index + 1) % (self.players.len() - 1)
    }

    fn next_player(&mut self) -> &mut Player {
        self.players.get_mut(self.next_player_index).unwrap()
    }

    fn advance_player(&mut self, player: &mut Player, throw: usize) {
        player.advance_to((player.current_position() + throw) % BOARD_SIZE)
    }

    fn is_game_over(&self) -> bool {
        self.player_won_index.is_some()
    }

    fn is_winner(player: Player) -> bool {
        player.score() == WINNING_SCORE
    }
}
