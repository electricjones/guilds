use crate::cards::{Eligibility, Playable};
use crate::state::State;

#[derive(Debug)]
pub struct Trinket {
    name: String,
}

impl Trinket {
    pub fn new(name: &str) -> Trinket {
        Trinket {
            name: name.to_string(),
        }
    }
}

impl Playable for Trinket {
    fn eligibility(&self, state: State) -> Eligibility {
        Eligibility::Eligible
    }

    fn play(&mut self, state: &mut State) -> Result<String, String> {
        println!("Playing {}", self.name);
        state.debug().push(self.name.clone());
        Ok(String::from("Done"))
    }
}
