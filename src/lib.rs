
//! A Game of rock-paper-scissors.


pub mod util;
pub mod game;




mod tests {

  use rand::Rng;

  // Test random value is 1, or 0!
  #[test]
  fn it_works() {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0, 2);
    assert_eq!(1, r);
  }
}
