// https://stackoverflow.com/questions/69210771/why-cant-an-identifier-start-with-a-number
#[path = "2022/mod.rs"]
mod twenty_twenty_two;
use twenty_twenty_two::{day1, day2, day3};

fn main() {
    day1::solution();
    day2::solution();
    day3::solution();
}
