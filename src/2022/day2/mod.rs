use rust_embed::RustEmbed;
use std::str;

// https://crates.io/crates/rust-embed
#[derive(RustEmbed)]
#[folder = "src/2022/day2/"]
struct Asset;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

// A X - Rock     - 1pt | Lose 0pt
// B Y - Paper    - 2pt | Draw 3pt
// C Z - Scissors - 3pt | Win  6pt
pub fn solution() {
    println!("https://adventofcode.com/2022/day/2");

    let file = Asset::get("input.txt").unwrap();

    let vec = file.data.to_owned();
    let res = str::from_utf8(&vec).unwrap();

    let rounds = res.lines();

    let mut sum = 0;
    for round in rounds.clone() {
        let parts: Vec<&str> = round.split(" ").collect();

        // ❌
        // let [a, b] = parts[..];

        // ✅ Destructuring a slice
        // > you might want to use `if let` to ignore the variants
        // > that aren't matched: `let (a, b) = if `, ` { (a, b) } else { todo!() }`
        // if let [a, b] = parts[..] {

        // }

        let points = match parts[..] {
            ["A", "X"] => ROCK + DRAW,
            ["A", "Y"] => PAPER + WIN,
            ["A", "Z"] => SCISSORS + LOSE,
            ["B", "X"] => ROCK + LOSE,
            ["B", "Y"] => PAPER + DRAW,
            ["B", "Z"] => SCISSORS + WIN,
            ["C", "X"] => ROCK + WIN,
            ["C", "Y"] => PAPER + LOSE,
            ["C", "Z"] => SCISSORS + DRAW,
            _ => 0, /* should not hit this arm */
        };

        sum += points;
    }

    // part 1
    println!("==> {}", sum); // 10994

    // part 2
    // X means you need to lose
    // Y means you need to draw
    // Z means you need to win.

    let mut sum = 0;
    for round in rounds.clone() {
        let parts: Vec<&str> = round.split(" ").collect();
        let points = match parts[..] {
            ["A", "X"] => LOSE + SCISSORS,
            ["A", "Y"] => DRAW + ROCK,
            ["A", "Z"] => WIN + PAPER,
            ["B", "X"] => LOSE + ROCK,
            ["B", "Y"] => DRAW + PAPER,
            ["B", "Z"] => WIN + SCISSORS,
            ["C", "X"] => LOSE + PAPER,
            ["C", "Y"] => DRAW + SCISSORS,
            ["C", "Z"] => WIN + ROCK,
            _ => 0, /* should not hit this arm */
        };

        sum += points;
    }
    println!("==> {}", sum) // 12526
}
