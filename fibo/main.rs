use std::io;

fn main() {
    let number = input("input fibonacci");
    println!("{} : {}", number, fibo(number));
    println!("iterator");
    (0..number).map(|x| fibo(x)).for_each(|r: u32| print!("{},", r));
}

fn input(msg: &str) -> u32 {
    let mut number = String::new();
    loop{
        println!("{}", msg);
        io::stdin().read_line(&mut number)
            .expect("fail");
        match number.trim().parse(){
            Ok(num) => return num,
            Err(_) => continue, 
        };
    }
}

fn fibo(num: u32) -> u32 {
    match num {
        0|1 => num,
        n => {
            let mut fibo = 0;
            let mut fibo_0 = 0;
            let mut fibo_1 = 1;
            for _ in 1..n {
                fibo = fibo_0 + fibo_1;
                fibo_0 = fibo_1;
                fibo_1 = fibo;
            }
            fibo
        }
    }
}