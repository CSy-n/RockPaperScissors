
//! A Game of rock-paper-scissors.


pub mod util;
pub mod game;



mod tests {

  //use rand::Rng;
  use crate::game::*;
  
  #[test]
  fn check_game_results() {

    assert_eq!(check_winner(GameOption::ROCK, GameOption::SCISSORS), GameResult::WIN);
    assert_eq!(check_winner(GameOption::ROCK, GameOption::ROCK), GameResult::DRAW);
    assert_eq!(check_winner(GameOption::ROCK, GameOption::PAPER), GameResult::LOSS);
    
    
    assert_eq!(check_winner(GameOption::SCISSORS, GameOption::PAPER), GameResult::WIN);
    assert_eq!(check_winner(GameOption::SCISSORS, GameOption::SCISSORS), GameResult::DRAW);
    assert_eq!(check_winner(GameOption::SCISSORS, GameOption::ROCK), GameResult::LOSS);
    
    
    assert_eq!(check_winner(GameOption::PAPER, GameOption::ROCK), GameResult::WIN);
    assert_eq!(check_winner(GameOption::PAPER, GameOption::PAPER), GameResult::DRAW);
    assert_eq!(check_winner(GameOption::PAPER, GameOption::SCISSORS), GameResult::LOSS);
  }
}
