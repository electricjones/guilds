use crate::cards::{Eligibility, Playable};
use crate::state::State;

pub struct Discovery {
    // NOTE: Can be done w/o a Box, but would require generics all the way up the line
    pub callable: Box<dyn FnMut(&mut State) -> Result<String, String>>,
}

impl Playable for Discovery {
    fn eligibility(&self, _state: State) -> Eligibility {
        Eligibility::Eligible
    }

    fn play(&mut self, state: &mut State) -> Result<String, String> {
        (self.callable)(state)
    }
}

pub fn traitor(state: &mut State) -> Result<String, String> {
    state.increment_round();
    Ok(String::from("Done"))
}