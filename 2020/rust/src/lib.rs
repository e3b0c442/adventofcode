use std::error::Error;

mod day1;
mod day2;
pub use day1::day1;
pub use day2::day2;

pub fn run_all(input_path: &str) -> Result<(), Box<dyn Error>> {
    let funcs = [day1, day2];
    for (i, func) in funcs.iter().enumerate() {
        func(&format!("{}/{}.txt", input_path, i + 1))?;
    }
    Ok(())
}
