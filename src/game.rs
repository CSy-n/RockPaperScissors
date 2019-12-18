

use rand::Rng;



#[allow(dead_code)]
#[derive(PartialEq)]
pub enum GameOption {
  ROCK, PAPER, SCISSORS, QUIT, INVALID
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum GameResult {
  WIN, LOSS, DRAW
}



/*
 Generate a Random RPS Item
*/

pub fn generate_random_option() -> GameOption {
    let option = GameOption::ROCK;
    let mut rng = rand::thread_rng();
  
    let option = match rng.gen_range(1, 5) {
        1    => GameOption::ROCK,
        2    => GameOption::PAPER,
        3    => GameOption::SCISSORS,
        4    => GameOption::QUIT,
        _    => panic!("Unexpected case!")
    };
    
    return option;
}

pub fn check_winner(left: GameOption, right: GameOption) -> GameResult {
  
  if left == GameOption::ROCK && right == GameOption::SCISSORS {
      return GameResult::WIN;
    } else if left == GameOption::ROCK && right == GameOption::ROCK {
      return GameResult::DRAW;
    } else if left == GameOption::ROCK && right == GameOption::PAPER {
      return GameResult::LOSS;
    } else if left == GameOption::SCISSORS && right == GameOption::PAPER {
      return GameResult::WIN;
    } else if left == GameOption::SCISSORS && right == GameOption::SCISSORS {
      return GameResult::DRAW;
    } else if left == GameOption::SCISSORS && right == GameOption::ROCK {
      return GameResult::LOSS;
    } else if left == GameOption::PAPER && right == GameOption::ROCK {
      return GameResult::WIN;
    } else if left == GameOption::PAPER && right == GameOption::PAPER {
      return GameResult::DRAW;
    } else if left == GameOption::PAPER && right == GameOption::SCISSORS {
      return GameResult::LOSS;
    }
    
    return GameResult::LOSS;
}


pub fn display_game_results(result: GameResult) {
  if result == GameResult::WIN {
      display_winner();
   } else if result == GameResult::DRAW {
      display_draw();
   } else {
      display_lost();
   }
}


pub fn print_banner() {
   println!("===============================");
   println!("Rock Paper Scissors!          |");
   println!("===============================");
   println!("1) Rock");
   println!("2) Paper");
   println!("3) Scissors");
   println!("4) Quit");
}

pub fn display_lost() {
   println!("===============================");
   println!(" You Lost, Try Again?         |");
   println!("===============================");
}

pub fn display_draw() {
   println!("===============================");
   println!("            |DRAW|            |");
   println!("===============================");
}

pub fn display_winner() {
   println!("==============================| )");
   println!("    <<<     WINNER!    >>>    | | ");
   println!("______________________________| )");
}
