use std::env;

pub fn get_filename() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    }
}