use std::error::Error;

mod day1;
mod day2;
mod day3;
pub use day1::day1;
pub use day2::day2;
pub use day3::day3;

pub fn run_all(input_path: &str) -> Result<(), Box<dyn Error>> {
    let funcs = [day1, day2, day3];
    for (i, func) in funcs.iter().enumerate() {
        func(&format!("{}/{}.txt", input_path, i + 1))?;
    }
    Ok(())
}
