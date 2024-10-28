#![deny(clippy::todo)]

use std::io::{stdin, stdout, Write};

use rand::{rngs::ThreadRng, thread_rng, Rng, RngCore};

fn main() {
    print!("Hello, welcome to hearts! Ready to play? (Y/n): ");
    stdout().flush();
    let mut ready_input = String::new();
    stdin()
        .read_line(&mut ready_input)
        .expect("Failed to read input.");
    if ready_input.trim().chars().next().is_some_and(|c| c == 'n') {
        return;
    }

    print!("How many players? ");
    stdout().flush().unwrap();
    let mut human_player_count = String::new();
    stdin()
        .read_line(&mut human_player_count)
        .expect("Failed to read input");
    let human_player_count = match human_player_count.trim().parse::<u8>() {
        Ok(4) => 4,
        Ok(n) if n > 0 && n <= 4 => {
            println!("Warning: AI is currently just a random player.");
            n
        }
        _ => panic!("Must be a number 1-4."),
    };

    GameState::new(human_player_count).run();
}

/// A player, who may be controlled by a human, an algorithm, or a test system
trait PlayerAgent {
    /// Take a turn and return the index of the card in their hand that they
    /// want to play
    fn turn(&mut self, hand: &Hand, current_score: i8) -> u8;
}

struct Player {
    agent: Box<dyn PlayerAgent>,
    score: i8,
    hand: Hand,
}

type Hand = Vec<Card>;
struct GameState {
    players: Vec<Player>,
    kitty: Option<Vec<Card>>,
}
struct Card {
    suit: Suit,
    value: u8,
}
impl Card {
    fn format_short(&self) -> String {
        let suit_char = match self.suit {
            Suit::Hearts => 'H',
            Suit::Diamonds => 'D',
            Suit::Spades => 'S',
            Suit::Clubs => 'C',
        };
        format!("{}{suit_char}", self.value)
    }
}
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl GameState {
    fn new(human_player_count: u8) -> Self {
        let mut player_agents: Vec<Box<dyn PlayerAgent>> = (0..human_player_count)
            .map(|_i| Box::new(HumanPlayer {}) as _)
            .collect::<Vec<_>>();
        player_agents
            .extend((0..(4 - human_player_count)).map(|_i| Box::new(RandomPlayer::new()) as _));
        let (hands, kitty) = deal();
        let players = player_agents
            .into_iter()
            .zip(hands)
            .map(|(agent, hand)| Player {
                agent,
                hand,
                score: 0,
            })
            .collect();
        let kitty = Some(kitty);
        Self { players, kitty }
    }
    fn run(mut self) {
        todo!()
    }
}

struct HumanPlayer;
impl PlayerAgent for HumanPlayer {
    fn turn(&mut self, hand: &Hand, current_score: i8) -> u8 {
        clear_screen();
        println!("You have {current_score} points.");
        println!("Here is your hand:");
        for (idx, card) in hand.iter().enumerate() {
            println!("{}: {}", idx + 1, card.format_short());
        }

        let mut card_choice = String::new();
        let mut first = true;
        while !card_choice
            .trim()
            .parse::<u8>()
            .is_ok_and(|n| n as usize <= hand.len())
        {
            if !first {
                println!("Sorry, that isn't a valid input.");
            } else {
                first = false;
            }

            card_choice.clear();
            print!("Which card would you like to play?\n> ");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut card_choice)
                .expect("Failed to get input");
        }

        card_choice.trim().parse::<u8>().unwrap() - 1
    }
}

// #[cfg(target_os="windows")]
// #[cfg(target_os="linux")]
fn clear_screen() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cls").status().unwrap();
    } else {
        std::process::Command::new("clear").status().unwrap();
    }
}

/// Deal out the deck. Return all the hands and the kitty as (hands, kitty)
fn deal() -> (Vec<Hand>, Vec<Card>) {
    //
}

struct RandomPlayer {
    rng: ThreadRng,
}
impl RandomPlayer {
    fn new() -> Self {
        let rng = thread_rng();
        Self { rng }
    }
}
impl PlayerAgent for RandomPlayer {
    fn turn(&mut self, hand: &Hand, _current_score: i8) -> u8 {
        self.rng.next_u32() as u8 % hand.len() as u8
    }
}
