#![deny(clippy::todo)]

use std::io::{stdin, stdout, Write};

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng, RngCore};

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
    fn turn(
        &mut self,
        hand: &Hand,
        current_score: i8,
        already_played: Vec<Card>,
        hearts_broken: bool,
    ) -> u8;
}

struct Player {
    agent: Box<dyn PlayerAgent>,
    score: i8,
    hand: Hand,
}

type Hand = Vec<Card>;
struct GameState {
    players: Vec<Player>,
    next_player: usize,
}
#[derive(Clone, Copy)]
struct Card {
    suit: Suit,
    value: u8,
}
#[derive(Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
impl Card {
    fn format_short(&self) -> String {
        let suit_char = match self.suit {
            Suit::Hearts => 'H',
            Suit::Diamonds => 'D',
            Suit::Spades => 'S',
            Suit::Clubs => 'C',
        };
        let value = match self.value {
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            n => n.to_string(),
        };
        format!("{value}{suit_char}")
    }
}

impl GameState {
    fn new(human_player_count: u8) -> Self {
        let mut player_agents: Vec<Box<dyn PlayerAgent>> = (0
            ..human_player_count)
            .map(|_i| Box::new(HumanPlayer {}) as _)
            .collect::<Vec<_>>();
        player_agents.extend(
            (0..(4 - human_player_count))
                .map(|_i| Box::new(RandomPlayer::new()) as _),
        );
        let hands = deal();
        let players = player_agents
            .into_iter()
            .zip(hands)
            .map(|(agent, hand)| Player {
                agent,
                hand,
                score: 0,
            })
            .collect();
        Self {
            players,
            next_player: 0,
        }
    }
    fn run(mut self) {
        loop {
            if self.run_turn() {
                break;
            }
        }
    }
    fn run_turn(&mut self) -> bool {
        let mut cards_played: Vec<Card> = Vec::new();
        for player in self.players.iter_mut() {
            //
        }
        todo!()
    }
}

struct HumanPlayer;
impl PlayerAgent for HumanPlayer {
    fn turn(
        &mut self,
        hand: &Hand,
        current_score: i8,
        already_played: Vec<Card>,
        hearts_broken: bool,
    ) -> u8 {
        clear_screen();
        println!("You have {current_score} points.");
        println!("Cards played already:");
        for card in already_played {
            // TODO: Maybe change this to be the long version? IDK
            println!("{}", card.format_short());
        }
        println!("Here is your hand:");
        for (idx, card) in hand.iter().enumerate() {
            println!("{}. {}", idx + 1, card.format_short());
        }

        let mut card_choice = String::new();
        let card_choice_parsed: u8;
        loop {
            card_choice.clear();
            print!("Which card would you like to play?\n> ");
            stdout().flush().unwrap();
            stdin()
                .read_line(&mut card_choice)
                .expect("Failed to get input");

            let Ok(choice_number) = card_choice.parse::<u8>() else {
                println!("You must enter a number.");
                continue;
            };

            match is_legal(hand, choice_number, hearts_broken) {
                LegalResult::Legal => {
                    card_choice_parsed = choice_number;
                    break;
                }
                LegalResult::OutOfRange => {
                    println!("That's not in your hand.")
                }
                LegalResult::HeartsNotYetBroken => {
                    println!(
                        "You can't play that yet; hearts haven't been broken."
                    )
                }
            }
        }

        card_choice_parsed
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

fn deal() -> Vec<Hand> {
    let mut deck = generate_deck();
    deck.as_mut_slice().shuffle(&mut thread_rng());
    let mut hands: Vec<Vec<Card>> = vec![Vec::with_capacity(13); 4];
    for _ in 0..13 {
        for hand in hands.iter_mut() {
            hand.push(deck.pop().unwrap());
        }
    }
    hands
}

fn generate_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    for suit in [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs].iter()
    {
        for value in 2..=14 {
            deck.push(Card {
                suit: suit.clone(),
                value,
            });
        }
    }

    deck
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
    fn turn(
        &mut self,
        hand: &Hand,
        _current_score: i8,
        _already_played: Vec<Card>,
        hearts_broken: bool,
    ) -> u8 {
        loop {
            let num = self.rng.next_u32() as u8 % hand.len() as u8;
            if let LegalResult::Legal = is_legal(hand, num, hearts_broken) {
                return num;
            }
        }
    }
}

enum LegalResult {
    Legal,
    OutOfRange,
    HeartsNotYetBroken,
}

fn is_legal(hand: &Hand, card_index: u8, hearts_broken: bool) -> LegalResult {
    let Some(card) = hand.get(card_index as usize) else {
        return LegalResult::OutOfRange;
    };
    if matches!(card.suit, Suit::Hearts) && !hearts_broken {
        return LegalResult::HeartsNotYetBroken;
    };
    LegalResult::Legal
}
