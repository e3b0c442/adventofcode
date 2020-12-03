use std::error::Error;

mod day1;
pub use day1::day1;

pub fn run_all(input_path: &str) -> Result<(), Box<dyn Error>> {
    let funcs = [day1];
    for (i, func) in funcs.iter().enumerate() {
        func(&format!("{}/{}.txt", input_path, i + 1))?;
    }
    Ok(())
}
