
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
    - 
  - Generate Random GameItem [Rock, Paper, Scissor]
  - Compare result
  - Display Results [Win, Loss]
  - Return to Menu [RPS Menu]
 


  Understanding the Module System


 */
 
 
use std::io::{self, Write};
use rock_paper_scissors::util;
use rock_paper_scissors::game::*;



fn main() {


  //Display Menu. . .    
  print_banner();

  print!(">>> ");
  io::stdout().flush().unwrap();

  //Request user input. . .
  let user_selection = util::read_line();

  //println!("user_selection={}", user_selection);
  
  //Check Valid option, otherwise try again.
  let game_move = parse_user_input(user_selection.as_str());
  

  //display_string_properties(user_selection.as_str());

  // Generate random Game Move
  let computer_move = generate_random_option();

  // Compare results:
  let game_result = check_winner(game_move, computer_move);

  //Display Results:

   display_game_results(game_result);
}


/** 
  Parse valid options
*/
fn parse_user_input(input: &str) -> GameOption {

  let option = match input { 
    "1" => GameOption::ROCK,
    "2" => GameOption::PAPER,
    "3" => GameOption::SCISSORS,
    "4" => GameOption::QUIT,
     _  => GameOption::INVALID
  };

  return option;
}

fn display_string_properties(input: &str) {
  println!("string={}",input);
}


