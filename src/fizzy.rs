use std::borrow::Cow;

const FIZZBUZZ_MAP: [(i32, &str); 3] = [(15, "FizzBuzz"), (3, "Fizz"), (5, "Buzz")];

pub trait Fizzy {
    /// Transforms the given input into its FizzBuzz counterpart
    fn fizzy(&self) -> Cow<'static, str>;
}

impl Fizzy for i32 {
    fn fizzy(&self) -> Cow<'static, str> {
        for (divisor, mapped_text) in FIZZBUZZ_MAP {
            if self % divisor == 0 {
                return Cow::Borrowed(mapped_text);
            }
        }
        Cow::Owned(self.to_string())
    }
}

pub fn play_fizzbuzz(iterator: impl IntoIterator<Item = i32>) -> String {
    let mut output = String::new();
    for i in iterator {
        // TODO: combine this?
        output.push_str(&i.fizzy());
        output.push('\n');
    }
    output.shrink_to_fit();
    output
}
