use simple_error::SimpleError;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    if let Some(path) = args.get(1) {
        return advent2015::day17(path);
    }
    Err(Box::new(SimpleError::new("No input file provided")))
}
