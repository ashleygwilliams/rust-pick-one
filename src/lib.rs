extern crate rand;

use rand::Rng;

/// Pick a random str from an Array of str
///
/// Example
///
/// ```
/// let example = ["doggo", "pupper", "smol", "<3"];
/// assert_eq!(pick_one_str(&example), "doggo");
/// ```
pub fn pick_one_str<'a>(collection: &'a [&'static str]) -> &'a str {
  let choice = rand::thread_rng().gen_range(0, collection.len());
  collection[choice]
}

#[test]
fn it_works() {
  let example = ["doggo"];
  assert_eq!(pick_one_str(&example), "doggo");
}  
