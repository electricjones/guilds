use crate::cards::discoveries::Discovery;
use crate::cards::trinkets::Trinket;
use crate::state::State;

pub mod discoveries;
pub mod trinkets;
pub mod wishes;

enum Eligibility {
    Eligible,
    NotEligible,
    // What others?
}

type PlayableAction = Box<dyn FnMut(&mut State) -> Result<String, String>>;

pub trait Playable {
    fn eligibility(&self, state: State) -> Eligibility;

    // fn play(&mut self, state: &mut State) -> Result<String, String>;
    fn play(&self) -> Vec<PlayableAction>;
}

#[derive(Debug)]
pub enum Cards {
    Discovery(Discovery),
    // Wish(Wish),
    // Charm(Charm),
    Trinket(Trinket),
}

impl Playable for Cards {
    fn eligibility(&self, _state: State) -> Eligibility {
        Eligibility::Eligible
        // TODO: Similar to play()
    }

    fn play(&self) -> Vec<PlayableAction> {
        // TODO: I probably don't need this match statement since they all implement Playable
        match self {
            Cards::Discovery(discovery) => discovery.play(),
            Cards::Trinket(trinkets) => trinkets.play(),
        }
    }
}

#[derive(Debug)]
pub struct Cost {
    silver: u8,
    ingredients: u8,
}
