use crate::cards::{Cards, Playable};
use crate::decks::CardStack;
use crate::state::State;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct PlayerDeck {
    cards: CardStack,
    deck: VecDeque<usize>,
    draw: VecDeque<usize>,
    discard: VecDeque<usize>,
}

impl From<CardStack> for PlayerDeck {
    fn from(cards: CardStack) -> Self {
        let length = cards.len();

        PlayerDeck {
            cards,
            deck: (0..length).collect(),
            draw: VecDeque::new(),
            discard: VecDeque::new(),
        }
    }
}

impl PlayerDeck {
    pub fn card(&mut self, index: &usize) -> Result<&mut Cards, String> {
        if let Some(card) = self.cards.get_mut(index.clone()) {
            return Ok(card);
        }

        Err(String::from("Card not found"))
    }

    pub fn hand(&mut self) -> Vec<usize> {
        self.draw.iter().cloned().collect()
    }

    pub fn draw(&mut self, count: usize) -> Result<Vec<usize>, String> {
        // If we don't have enough, cycle first
        if self.deck.len() < count {
            self.cycle();
        }

        self.draw.extend(self.deck.drain(..count));
        Ok(self.draw.make_contiguous()[..].to_vec())
    }

    pub fn discard(&mut self, card_id: &usize) -> Result<(), String> {
        // Find the card in deck and move it to discard
        if let Some(pos) = self.deck.iter().position(|&x| x == *card_id) {
            self.deck.remove(pos);
            self.discard.push_back(card_id.clone());
        }

        // Find the card in draw and move it to discard
        if let Some(pos) = self.draw.iter().position(|&x| x == *card_id) {
            self.draw.remove(pos);
            self.discard.push_back(card_id.clone())
        }

        Ok(())
    }

    pub fn play(&mut self, index: &usize, _state: &mut State) -> Result<String, String> {
        let card = self.cards.get_mut(index.clone()).unwrap();
        match card {
            Cards::Trinket(trinket) => trinket.play(),
            _ => return Err("Nope".to_string()),
        };

        Ok("Okay".to_string())
    }

    pub fn cycle(&mut self) {
        // Shuffle the discard pile
        let mut rng = thread_rng();

        let mut discard_vec: Vec<usize> = self.discard.drain(..).collect();
        discard_vec.shuffle(&mut rng);

        // Move that do the deck
        self.deck.extend(discard_vec.drain(..));
    }
}
