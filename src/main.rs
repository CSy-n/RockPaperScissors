
use std::io::{self, Write};
use RockPaperScissors::util;

fn main() {
    
  print_banner();

  print!(">> ");
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

