use std::fmt;

enum FizzBuzz {
    FizzBuzz,
    Buzz,
    Fizz,
    Num(u32),
}

impl fmt::Display for FizzBuzz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FizzBuzz::FizzBuzz => write!(f, "FizzBuzz"),
            FizzBuzz::Buzz => write!(f, "Buzz"),
            FizzBuzz::Fizz => write!(f, "Fizz"),
            FizzBuzz::Num(n) => write!(f, "{}", n),
        }
    }
}

impl From<u32> for FizzBuzz {
    fn from(n: u32) -> FizzBuzz {
        match (n%3, n%5) {
            (0, 0) => FizzBuzz::FizzBuzz,
            (_, 0) => FizzBuzz::Buzz,
            (0, _) => FizzBuzz::Fizz,
            _ => FizzBuzz::Num(n),
        }
    }
}

fn main() {
    (1..30).map(|x| x.into()).for_each(|r: FizzBuzz| println!("{}",r));
}