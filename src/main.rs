
/**
  
  Rock Paper Scissors:
  :::::::::::::::::::::


  A Rock Paper Scissors game, rules:
   Rock beats Scissors
   Paper beats Rock
   Scissors beat Paper

  
  <|RPS Menu|>

  ===============================
  Rock Paper Scissors!
  ===============================
  1) Rock
  2) Paper
  3) Scissors
  >>> <X>

  |>RPS MENU<|

  - Display Menu [RPS Menu]
  - Take input from user
  - Generate Random GameItem [Rock, Paper, Scissor]
  - Compare result
  - Display Results [Win, Loss]
  - Return to Menu [RPS Menu]
 


  Understanding the Module System


 */

use std::io::{self, Write};
use rand::Rng;
use RockPaperScissors::util;

enum MenuOption {
  ROCK, PAPER, SCISSORS, QUIT
}

enum GameItem {
  ROCK, PAPER, SCISSORS
}


/*
  TODO:

  Echo user result back to them. . .
*/
fn main() {
    
  print_banner();
  display_winner();
  display_lost();

  print!(">>> ");
  io::stdout().flush();

  generate_game_item();

  util::read_line();

}

/*
 Generate a Random RPS Item
*/

fn generate_game_item() -> GameItem {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 3);
    GameItem::ROCK
}



fn print_banner() {
   println!("===============================");
   println!("Rock Paper Scissors!");
   println!("===============================");
   println!("1) Rock");
   println!("2) Paper");
   println!("3) Scissors");
   println!("4) Quit");
}

fn display_winner() {
   println!("============================== )");
   println!(" 	<<<	WINNER!    >>>>   | |");
   println!("============================== )");


}

fn display_lost() {
   println!("===============================");
   println!(" You Lost, Try Again?           |");
   println!("===============================");


}
