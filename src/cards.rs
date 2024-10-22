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
pub trait Playable {
    fn eligibility(&self, state: State) -> Eligibility;

    // TODO: Return better results
    fn play(&mut self, state: &mut State) -> Result<String, String>;
}

#[derive(Debug)]
pub enum Cards {
    Discovery(Discovery),
    // Wish(Wish),
    // Charm(Charm),
    Trinket(Trinket),
}

#[derive(Debug)]
pub struct Cost {
    silver: u8,
    ingredients: u8,
}
