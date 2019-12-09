
use std::io::{self, Write};
use RockPaperScissors::util;




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
 
 */

enum GameItem {
  ROCK, PAPER, SCISSORS
}

fn main() {
    
  print_banner();

  print!(">>> ");
  io::stdout().flush();


  util::read_line();

}






fn print_banner() {
   println!("===============================");
   println!("Rock Paper Scissors!");
   println!("===============================");
   println!("1) Rock");
   println!("2) Paper");
   println!("3) Scissors");
}

