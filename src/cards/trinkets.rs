use crate::cards::{Eligibility, Playable, PlayableAction};
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
    fn eligibility(&self, _state: State) -> Eligibility {
        Eligibility::Eligible
    }

    fn play(&self) -> Vec<PlayableAction> {
        // TODO: Not sure how happy I am with this setup, but I think its working
        let name_clone = self.name.clone();
        let name_clone2 = self.name.clone();

        vec![
            Box::new(move |state| {
                println!("Trinket action 1 for {}", name_clone);
                state.increment_round();
                Ok(String::from("good".to_string()))
            }),
            Box::new(move |state| {
                println!("Trinket action 2 for {}", name_clone2);
                state.increment_round();
                Ok(String::from("good2"))
            }),
        ]
    }
}
