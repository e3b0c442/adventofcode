use std::error::Error;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
pub use day1::day1;
pub use day10::day10;
pub use day11::day11;
pub use day12::day12;
pub use day13::day13;
pub use day14::day14;
pub use day15::day15;
pub use day16::day16;
pub use day17::day17;
pub use day18::day18;
pub use day19::day19;
pub use day2::day2;
pub use day20::day20;
pub use day3::day3;
pub use day4::day4;
pub use day5::day5;
pub use day6::day6;
pub use day7::day7;
pub use day8::day8;
pub use day9::day9;

pub fn run_all(input_path: &str) -> Result<(), Box<dyn Error>> {
    let funcs = [
        day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13, day14,
        day15, day16, day17, day18, day19, day20,
    ];
    for (i, func) in funcs.iter().enumerate() {
        func(&format!("{}/{}.txt", input_path, i + 1))?;
    }
    Ok(())
}
