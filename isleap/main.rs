use std::io;
use std::fmt;

enum Isleap {
    Leap,
    Not,
}
impl From<u32> for Isleap {
    fn from(isleap: u32) -> Isleap {
        match (isleap%4 == 0 && isleap%100 != 0) || isleap%400 == 0 {
            true => Isleap::Leap,
            false => Isleap::Not,
        }
    }
}
impl fmt::Display for Isleap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Isleap::Leap => write!(f, "閏年！"),
            Isleap::Not => write!(f, "ちゃうよ！"),
        }
    }
}

fn main() {
    let isleap: Isleap = ret_input().into();
    println!("{}", isleap);
}

fn ret_input() -> u32 {
    println!("閏年判定を行います。判定したい年を入力して下さい.");
    let mut year = String::new();
    io::stdin().read_line(&mut year)
        .expect("fail");
    let year: u32 = year.trim().parse()
        .expect("fail");
    year
}