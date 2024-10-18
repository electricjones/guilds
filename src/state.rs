use std::collections::HashMap;

mod board;
mod players;
mod market;
mod points;

#[derive(TypedBuilder, Debug, Clone)]
pub struct State {
    // players: String,
    // playfield: Playfield,
    // market: String,
    // wishes: String
    #[builder(default=1)]
    round: i32,

    players: HashMap<PlayerId, Player>,
    player_order: PlayerOrder,

    /// Should be the first player in the play order
    // TODO: Shouldn't need to set this explicitly
    active_player: PlayerId,
}

impl State {
    pub(crate) fn increment_round(&mut self) {
        self.round += 1;
        
    }
}

impl State {
    pub fn active_player(&self) -> PlayerId {
        self.active_player
    }

    pub fn round(&self) -> i32 {
        self.round
    }

    pub fn player_order(&self) -> &PlayerOrder {
        &self.player_order
    }

    pub fn player_order_mut(&mut self) -> &mut PlayerOrder {
        &mut self.player_order
    }

    pub fn players(&self) -> &HashMap<PlayerId, Player> {
        &self.players
    }
}

type PlayerId = u8;

#[derive(TypedBuilder, Debug, Clone)]
pub struct Player {
    name: String,
    id: u8,
}

impl Player {
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn set_id(&mut self, id: u8) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = name.into();
    }
}

use std::collections::VecDeque;
use typed_builder::TypedBuilder;

// TODO: Move this to its own module
// TODO: Test all these From's
// TODO: I may need that last FROM
#[derive(TypedBuilder, Debug, Clone)]
pub struct PlayerOrder {
    order: VecDeque<PlayerId>,
    current_index: usize,
}

impl From<Vec<PlayerId>> for PlayerOrder {
    fn from(order: Vec<PlayerId>) -> Self {
        PlayerOrder {
            order: order.into_iter().collect(),
            current_index: 0,
        }
    }
}

impl<const N: usize> From<[PlayerId; N]> for PlayerOrder {
    fn from(order: [PlayerId; N]) -> Self {
        PlayerOrder {
            order: order.into_iter().collect(),
            current_index: 0,
        }
    }
}

impl From<&[PlayerId]> for PlayerOrder {
    fn from(order: &[PlayerId]) -> Self {
        PlayerOrder {
            order: order.into_iter().copied().collect(),
            current_index: 0,
        }
    }
}

// impl From<Vec<PlayerId>> for PlayerOrder {
//     fn from(order: Vec<PlayerId>) -> Self {
//         PlayerOrder {
//             order,
//             current_index: 0,
//         }
//     }
// }

impl PlayerOrder {
    pub fn new(order: Vec<PlayerId>) -> Self {
        PlayerOrder {
            order: order.into_iter().collect(),
            current_index: 0,
        }
    }

    pub fn next(&mut self) -> Option<PlayerId> {
        if self.order.is_empty() {
            return None;
        }

        if self.current_index >= self.order.len() {
            self.current_index = 0;
            return None;
        }

        let player_id = self.order[self.current_index];
        self.current_index += 1;

        Some(player_id)
    }

    pub fn reset_round(&mut self) {
        self.current_index = 0;
    }

    pub fn set_order(&mut self, new_order: Vec<PlayerId>, new_round: u32) {
        self.order = new_order.into_iter().collect();
        self.current_index = 0;
    }
}

// #[derive(Clone, Builder, Debug)]
// #[builder(setter(into))]
// pub struct Playfield {
//     rows: Vec<CardStack>,
// }
//
// #[derive(Clone, Builder, Debug)]
// #[builder(setter(into))]
// pub struct CardStack {
//     stack: Vec<Card>,
//     size: usize,
// }
//
// #[derive(Debug, Clone)]
// pub enum Card {
//     Discovery(DiscoveryCard),
//     Wish,
//     Market,
//     Trinket,
//     Charm,
// }
//
// #[derive(Clone, Builder, Debug)]
// #[builder(setter(into))]
// pub struct DiscoveryCard {
//     facing: Facing,
//     flavor: String,
//
// }
//
// #[derive(Clone, Debug)]
// pub enum Facing {
//     Up,
//     Down,
// }
