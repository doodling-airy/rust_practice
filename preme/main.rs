use std::io;

fn main() {
    let fig: u32 = input_recieve();
    let is_preme: bool = preme_check(fig);
    match is_preme {
        false => println!("素数ではない！"),
        true  => println!("素数！"),
    };
}

fn input_recieve() -> u32 {
    let mut fig = String::new();
    io::stdin().read_line(&mut fig)
        .expect("fail");
    let fig: u32 = fig.trim().parse()
        .expect("fail");
    fig
}

fn preme_check(fig: u32) -> bool {
    match fig {
        0|1 => return false,
        2   => return true,
        _   => return more_three(fig),
    };
}

fn more_three(fig: u32) -> bool {
    if fig%2 == 0 { return false; }

    let mut a = 3;
    while a * a <= fig {
        if fig%a == 0 { return false; }
        a = a+2;
    }
    return true;
} 