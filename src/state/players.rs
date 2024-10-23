use crate::decks::player::PlayerDeck;
use std::collections::VecDeque;
use typed_builder::TypedBuilder;

pub type PlayerId = u8;

#[derive(TypedBuilder, Debug)]
pub struct Player {
    name: String,
    id: u8,
    deck: PlayerDeck,
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

    pub fn deck(&mut self) -> &mut PlayerDeck {
        &mut self.deck
    }
}

// TODO: Move this to its own module
// TODO: Test all these From's
// TODO: I may need that last FROM
#[derive(TypedBuilder, Debug, Clone)]
pub struct PlayerOrder {
    order: VecDeque<PlayerId>,
    current_index: i8,
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
        PlayerOrder::new(order.into_iter().collect())
    }
}

impl From<&[PlayerId]> for PlayerOrder {
    fn from(order: &[PlayerId]) -> Self {
        PlayerOrder::new(order.into_iter().copied().collect())
    }
}

impl PlayerOrder {
    pub fn new(order: Vec<PlayerId>) -> Self {
        PlayerOrder {
            order: order.into_iter().collect(),
            // TODO: Not sure I like starting at -1, but I think its okay for now.
            //       - this is the best way to ensure we start at player 1 in the first round
            current_index: -1,
        }
    }

    pub fn next(&mut self) -> Option<PlayerId> {
        if self.order.is_empty() {
            return None;
        }

        self.current_index += 1;

        if self.current_index as usize >= self.order.len() {
            self.current_index = -1;
            return None;
        }

        let player_id = self.order[self.current_index as usize];

        Some(player_id)
    }

    pub fn reset_round(&mut self) {
        self.current_index = 0;
    }

    pub fn set_order(&mut self, new_order: Vec<PlayerId>, _new_round: u32) {
        self.order = new_order.into_iter().collect();
        self.current_index = 0;
    }
}
