use crate::cards::{Cost, Eligibility, Playable, PlayableAction};
use crate::state::State;
use std::fmt;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct Discovery {
    id: u8,
    count: u8,
    name: String,
    flavor: String,
    cost: Cost,

    pub callable: Box<dyn FnMut(&mut State) -> Result<String, String>>,
    pub eligible: Box<dyn Fn(&State) -> Eligibility>,
}

impl fmt::Debug for Discovery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Discovery")
            .field("id", &self.id)
            .field("count", &self.count)
            .field("name", &self.name)
            .field("flavor", &self.flavor)
            .field("cost", &self.cost)
            // TODO: I can probably do something more interesting here
            .field("callable", &"callable()")
            .field("eligible", &"eligible()")
            .finish()
    }
}

impl Playable for Discovery {
    fn eligibility(&self, _state: State) -> Eligibility {
        todo!()
    }

    fn play(&self) -> Vec<PlayableAction> {
        todo!()
    }
}

// pub fn traitor(state: &mut State) -> Result<String, String> {
//     state.increment_round();
//     Ok(String::from("Done"))
// }
