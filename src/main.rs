trait Fizzy {
    fn fizzify(&self) -> String;
}

impl Fizzy for i32 {
    fn fizzify(&self) -> String {
        match (self % 3, self % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => self.to_string()
        }
    }
}

fn main() {
    for i in 1..=100 {
        println!("{}", i.fizzify());
    }
}