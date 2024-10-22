use crate::cards::trinkets::Trinket;
use crate::cards::{Cards, Playable};
use crate::decks::player::PlayerDeck;
use crate::state::State;
use maplit::hashmap;
use state::players::Player;

mod cards;
mod characters;
mod decks;
mod players;
mod state;
mod ui;
mod utilities;

enum Commands {
    A,
    B,
    C,
    D,
    E,
    F,
    Unknown,
}

impl From<&str> for Commands {
    fn from(value: &str) -> Self {
        match value {
            "a" | "A" => Commands::A,
            "b" | "B" => Commands::B,
            "c" | "C" => Commands::C,
            "d" | "D" => Commands::D,
            "e" | "E" => Commands::E,
            "f" | "F" => Commands::F,
            _ => Commands::Unknown,
        }
    }
}

fn main() {
    // Create the players
    let michael_cards = PlayerDeck::from(vec![
        Cards::Trinket(Trinket::new("Warrior")),
        Cards::Trinket(Trinket::new("Monk")),
        Cards::Trinket(Trinket::new("Cleric")),
        Cards::Trinket(Trinket::new("Minister")),
        Cards::Trinket(Trinket::new("Merchant")),
    ]);

    // Player::builder().name("Michael".into()).id(1).deck(michael_cards).build();

    // let james_card = PlayerDeck::from(vec![
    //     Cards::Trinket(Trinket::new("Warrior2")),
    //     Cards::Trinket(Trinket::new("Monk2")),
    //     Cards::Trinket(Trinket::new("Cleric2")),
    //     Cards::Trinket(Trinket::new("Minister2")),
    //     Cards::Trinket(Trinket::new("Merchant2")),
    // ]);
    //
    // let lori_cards = PlayerDeck::from(vec![
    //     Cards::Trinket(Trinket::new("Warrior3")),
    //     Cards::Trinket(Trinket::new("Monk3")),
    //     Cards::Trinket(Trinket::new("Cleric3")),
    //     Cards::Trinket(Trinket::new("Minister3")),
    //     Cards::Trinket(Trinket::new("Merchant3")),
    // ]);

    let players = hashmap! {
        1 => Player::builder().name("Michael".into()).id(1).deck(michael_cards).build(),
        // 2 => Player::builder().name("James".into()).id(2).deck(james_card).build(),
        // 3 => Player::builder().name("Lori".into()).id(3).deck(lori_cards).build(),
    };

    // Initialize the state
    // Create state from Players, Scenario, ect
    let mut state = State::builder()
        .round(1)
        .players(players)
        .player_order([2, 1, 3].into())
        .active_player(2)
        .debug(Vec::new())
        .build();

    let active_player_id = 1;

    // Borrow state to get the deck
    let active_player = &mut state.players().get_mut(&active_player_id).unwrap();
    let deck = active_player.deck();
    println!("Starting: {:#?}", deck);

    // Draw 3 cards
    let hand = deck.draw(3).unwrap();
    println!("Hand: {:#?}", hand);
    println!("After Draw: {:#?}", deck);

    // Play the cards
    for card_id in &hand {
        let actions = state
            .players()
            .get_mut(&active_player_id)
            .unwrap()
            .deck()
            .card(card_id)
            .unwrap()
            .play();

        for mut action in actions {
            action(&mut state).unwrap();
        }
    }

    // Discard the cards
    for card_id in &hand {
        state
            .players()
            .get_mut(&active_player_id)
            .unwrap()
            .deck()
            .discard(card_id)
            .unwrap();
    }

    let active_player = &mut state.players().get_mut(&active_player_id).unwrap();
    let deck = active_player.deck();
    println!("Ending: {:#?}", deck);

    // Cycle the Cards
    state
        .players()
        .get_mut(&active_player_id)
        .unwrap()
        .deck()
        .cycle();

    let active_player = &mut state.players().get_mut(&active_player_id).unwrap();
    let deck = active_player.deck();
    println!("Ending: {:#?}", deck);

    // Draw three again
    let active_player = &mut state.players().get_mut(&active_player_id).unwrap();
    let deck = active_player.deck();
    let hand = deck.draw(3).unwrap();
    println!("Hand: {:#?}", hand);
    println!("After Second Draw: {:#?}", deck);

    // This is just my attempt to manage player order in the most basic way possible.
    // This is not at all the real input scheme. Just a few stupid commands to test some ideas

    // Run the game loop
    // loop {
    //     // TODO: I think this will start with Player 2
    //     let active_player_id = match state.player_order_mut().next() {
    //         None => {
    //             // If we have finished this round, move to the next and start again
    //             println!("Finished round: {}", state.round());
    //             state.increment_round();
    //             println!("Beginning round: {}", state.round());
    //
    //             continue;
    //         }
    //         Some(player_id) => player_id,
    //     };
    //
    //     let active_player = &mut state.players().get_mut(&active_player_id).unwrap();
    //
    //     println!("Active Player: {}", active_player.name());
    //     println!("Acceptable commands: A-F");
    //
    //     loop {
    //         print!("Enter command: ");
    //         io::stdout().flush().unwrap();
    //
    //         let mut input = String::new();
    //         io::stdin().read_line(&mut input).unwrap();
    //         let input = input.trim();
    //
    //         let command: Commands = input.into();
    //
    //         match command {
    //             Commands::A => {
    //                 let me = false;
    //
    //                 let deck = active_player.deck();
    //                 // Starting
    //                 println!("{:#?}", deck);
    //
    //                 // Draw 3 cards
    //                 let drawn = deck.draw(3).unwrap();
    //                 println!("{:#?}", drawn);
    //                 println!("{:#?}", deck);
    //
    //                 // Play those three cards and discard
    //                 for card_id in drawn {
    //                     deck.play(&card_id, &mut state).unwrap();
    //                     // deck.discard(&card_id).unwrap()
    //                 }
    //
    //                 // Check the ending
    //                 println!("{:#?}", deck);
    //             }
    //             Commands::B => println!("Player '{}' entered command: B", active_player.name()),
    //             Commands::C => println!("Player '{}' entered command: C", active_player.name()),
    //             Commands::D => println!("Player '{}' entered command: D", active_player.name()),
    //             Commands::E => println!("Player '{}' entered command: E", active_player.name()),
    //             Commands::F => println!("Player '{}' entered command: F", active_player.name()),
    //             Commands::Unknown => {
    //                 println!("Invalid command. Please enter one of A-F");
    //                 continue;
    //             }
    //         }
    //
    //         break;
    //     }
    // }
}
