use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let message = &args[1];
        println!("{}", message);
    }
}
