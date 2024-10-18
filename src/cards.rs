use crate::cards::discoveries::Discovery;
use crate::state::State;

pub mod discoveries;
pub mod wishes;

enum Eligibility {
    Eligible,
    NotEligible,
    // What others?
}
pub trait Playable {
    fn eligibility(&self, state: State) -> Eligibility;

    // TODO: Return better results
    fn play(&mut self, state: &mut State) -> Result<String, String>;
}

enum Cards {
    Discovery(Discovery),
    // Wish(Wish),
    // Charm(Charm),
    // Trinket(Trinket),
}

pub struct Cost {
    silver: u8,
    ingredients: u8,
}
