extern crate rand;

use rand::Rng;
use std::io;

// gamemate
enum Gamemate {
    Player(Vec<u8>),
    Dealer(Vec<u8>),
}
impl Gamemate {
    fn draw_card(&mut self, card: &mut Vec<u8>) {
        loop {
            let draw_number: u8 = rand::thread_rng().gen_range(1, 14);
            let draw_usize: usize = draw_number.into();
            
            if card[draw_usize - 1] > 3 { continue; }
            else { card[draw_usize -1] += 1; }

            match self {
                Gamemate::Player(have) => have.append(&mut vec![draw_number]),
                Gamemate::Dealer(have) => have.append(&mut vec![draw_number]),
            }
            break;
        }
        self.show_card();
    }
    fn show_card(&self) {
        match self {
            Gamemate::Player(have) => println!("player : {:?}", have),
            Gamemate::Dealer(have) => println!("dealer : {:?}", have),
        }
    }
    fn real_sum(&self) -> u8 {
        match self {
            Gamemate::Player(have) => have.iter().fold(0, |sum, a| sum + a),
            Gamemate::Dealer(have) => have.iter().fold(0, |sum, a| sum + a),
        }
    }
    fn turn(&mut self, mut card: &mut Vec<u8>) {
        match self {
            Gamemate::Player(_) => {
                        loop{
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("fail");
                            let input: &str = &input;
                            match input.trim() {
                                "d" => self.draw_card(&mut card),
                                "f" => {
                                    println!("{}", self.real_sum());
                                    break;
                                },
                                _ => continue,
                            }
                        }
            },
            Gamemate::Dealer(_) => {
                loop{
                    if self.real_sum() < 17 {
                        self.draw_card(&mut card);
                    }else {
                        break;
                    }
                }
                println!("{}", self.real_sum());
            },
        }
    }
}
// game
struct Game {
    player: Gamemate,
    dealer: Gamemate,
    card: Vec<u8>,
}
impl Game {
    fn new() -> Game {
        let mut player = Gamemate::Player(vec![]);
        let mut dealer = Gamemate::Dealer(vec![]);
        let mut card = vec![0; 13];
        //prestart
        player.draw_card(&mut card);
        dealer.draw_card(&mut card);
        player.draw_card(&mut card);
        dealer.draw_card(&mut card);
        Game {
            player,
            dealer,
            card,
        }
    }
    fn start(&mut self) {
        self.player.turn(&mut self.card);
        self.dealer.turn(&mut self.card);
    }
}
// card

//main
fn main() {
    let mut game = Game::new();
    game.start();
}
