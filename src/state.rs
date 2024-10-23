use crate::cards::{Playable, PlayableAction};
use crate::decks::player::PlayerDeck;
use crate::state::players::{Player, PlayerId, PlayerOrder};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

pub mod board;
pub mod market;
pub mod players;
pub mod points;

// TODO: This will eventually track
//      - Generations, revisions, watchers, etc
#[derive(TypedBuilder, Debug)]
pub struct State {
    #[builder(default = 1)]
    round: i32,

    debug: Vec<String>,

    players: HashMap<PlayerId, Player>,
    player_order: PlayerOrder,

    /// Should be the first player in the play order
    // TODO: Shouldn't need to set this explicitly
    active_player: PlayerId,
}

impl State {
    pub fn increment_round(&mut self) {
        self.round += 1;
    }
    pub fn active_player(&self) -> PlayerId {
        self.active_player
    }

    pub fn round(&self) -> i32 {
        self.round
    }

    pub fn debug(&mut self) -> &mut Vec<String> {
        &mut self.debug
    }

    pub fn player_order(&self) -> &PlayerOrder {
        &self.player_order
    }

    pub fn player_order_mut(&mut self) -> &mut PlayerOrder {
        &mut self.player_order
    }

    pub fn players(&mut self) -> &mut HashMap<PlayerId, Player> {
        &mut self.players
    }
}

impl State {
    pub fn player_deck(&mut self, player_id: PlayerId) -> Option<&mut PlayerDeck> {
        Some(self.players().get_mut(&player_id).unwrap().deck())
    }

    pub fn play_player_card(&mut self, player_id: PlayerId, card_id: usize) -> Vec<PlayableAction> {
        self.players()
            .get_mut(&player_id)
            .unwrap()
            .deck()
            .card(&card_id)
            .unwrap()
            .play()
    }
}
