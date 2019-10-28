extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::env;

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 { panic!("too little args!"); }

    let module_name = &args[1];

    let input_count = &args[2];
    let input_count: u8 = input_count.trim().parse().expect("fail");
    let output_count = &args[3];
    let output_count: u8 = output_count.trim().parse().expect("fail");

    let format_input: Vec<String> = gen_content(input_count, "input H");
    let format_output: Vec<String> = gen_content(output_count, "output N");

    ctx.set_contents(format!("module {}({}, {});", module_name, format_input.join(", "), format_output.join(", ")).to_owned()).unwrap();
}

fn gen_content(count: u8, sent: &str) -> Vec<String> {
    let mut ar: Vec<String> = vec![];
    for i in 1..count + 1 {
        ar.append(&mut vec![format!("{}{:02}", sent, i)]);
    }
    return ar
}
