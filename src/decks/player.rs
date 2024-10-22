use crate::cards::Cards;
use crate::decks::CardStack;
use crate::utilities::VecExt;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct PlayerDeck {
    deck: CardStack,
    draw: CardStack,
    discard: CardStack,
}

impl From<CardStack> for PlayerDeck {
    fn from(cards: CardStack) -> Self {
        PlayerDeck {
            deck: cards,
            draw: Vec::new(),
            discard: Vec::new(),
        }
    }
}

impl PlayerDeck {
    // TODO: This may be able to be abstracted to a trait
    pub fn draw(&mut self, count: usize) -> Result<&mut [Cards], String> {
        // If we don't have enough, cycle first
        if self.deck.len() < count {
            self.cycle();
        }

        Ok(&mut self.deck[0..count])
    }

    pub fn discard(&mut self, index: usize) -> Result<(), String> {
        if let Some(card) = self.draw.remove_safe(index) {
            self.discard.push(card);
            return Ok(());
        }

        Err(String::from("Card not found"))
    }

    pub fn cycle(&mut self) {
        // Shuffle the discard pile
        let mut rng = thread_rng();
        self.discard.shuffle(&mut rng);

        // Move that do the deck
        self.deck.extend(self.discard.drain(..));
    }
}
