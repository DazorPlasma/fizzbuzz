mod fizzy;
use fizzy::*;

fn main() {
    println!("{}", fizzy::play_fizzbuzz(0..=100));
    println!("{}", 12345.fizzy());
}
