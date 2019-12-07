
extern crate rockpaperscissors;

use std::io;
use std::io::{Write};

use rockpaperscissors::util;

fn main() {
    
  print_banner();

  print!(">> ");
  io::stdout().flush();


  read_line();

}

fn print_banner() {
   println!("===============================");
   println!("Rock Paper Scissors!");
   println!("===============================");
   println!("1) Rock");
   println!("2) Paper");
   println!("3) Scissors");
}

