
use std::io;
use std::io::{Write};


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



fn read_line() -> String {
    let mut read = String::new();
    io::stdin().read_line(&mut read).expect("Unable to read line.");
    read.pop(); //remove the \n
    read
}
