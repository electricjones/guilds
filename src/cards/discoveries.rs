use crate::cards::{Cost, Eligibility, Playable};
use crate::state::State;

pub struct Discovery {
    id: u8,
    count: u8,
    name: String,
    flavor: String,
    cost: Cost,

    // NOTE: Can be done w/o a Box, but would require generics all the way up the line
    pub callable: Box<dyn FnMut(&mut State) -> Result<String, String>>,
    pub eligible: Box<dyn Fn(&State) -> Eligibility>,
}

impl Playable for Discovery {
    fn eligibility(&self, state: &State) -> Eligibility {
        (self.eligible)(state)
    }

    fn play(&mut self, state: &mut State) -> Result<String, String> {
        (self.callable)(state)
    }
}

pub fn traitor(state: &mut State) -> Result<String, String> {
    state.increment_round();
    Ok(String::from("Done"))
}
