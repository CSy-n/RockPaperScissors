
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
use rock_paper_scissors::util;
use rock_paper_scissors::game::*;



/*
  TODO:

  Echo user result back to them. . .
*/
fn main() {
    
  print_banner();

  print!(">>> ");
  io::stdout().flush().unwrap();

  generate_game_option();

  util::read_line();

}





