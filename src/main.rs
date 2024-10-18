use maplit::hashmap;
use std::io::{self, Write};
use std::str::FromStr;
use crate::state::{PlayerBuilder, PlayerOrder, StateBuilder};

mod cards;
mod state;
mod players;
mod characters;
mod ui;

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
    let players = hashmap! {
        1 => PlayerBuilder::default().name("Michael".into()).id(1).build().unwrap(),
        2 => PlayerBuilder::default().name("James".into()).id(2).build().unwrap(),
        3 => PlayerBuilder::default().name("Lori".into()).id(3).build().unwrap(),
    };

    let mut state = StateBuilder::default()
        .round(1)
        .players(players)
        .player_order::<PlayerOrder>([2, 1, 3].into())
        .active_player(2)
        .build().unwrap();

    // This is just my attempt to manage player order in the most basic way possible.
    // This is not at all the real input scheme. Just a few stupid commands to test some ideas

    loop {
        let active_player_id = match state.player_order_mut().next() {
            None => {
                // If we have finished this round, move to the next and start again
                println!("Finished round: {}", state.round());
                state.increment_round();
                println!("Beginning round: {}", state.round());

                continue;
            }
            Some(player_id) => player_id
        };

        let active_player = &state.players()[&active_player_id];

        println!("Active Player: {}", active_player.name());
        println!("Acceptable commands: A-F");

        loop {
            print!("Enter command: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            let command: Commands = input.into();

            match command {
                Commands::A => println!("Player '{}' entered command: A", active_player.name()),
                Commands::B => println!("Player '{}' entered command: B", active_player.name()),
                Commands::C => println!("Player '{}' entered command: C", active_player.name()),
                Commands::D => println!("Player '{}' entered command: D", active_player.name()),
                Commands::E => println!("Player '{}' entered command: E", active_player.name()),
                Commands::F => println!("Player '{}' entered command: F", active_player.name()),
                Commands::Unknown => {
                    println!("Invalid command. Please enter one of {:?}", commands);
                    continue;
                }
            }

            break;
        }
    }
}