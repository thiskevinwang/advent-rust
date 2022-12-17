use rust_embed::RustEmbed;
use std::str;

use std::collections::HashSet;

// The English alphabet as a vector of characters in Rust
// https://stackoverflow.com/a/19061862/9823455
//
// ...begin at index 1 for convenience
static ASCII: [char; 53] = [
    '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

// https://crates.io/crates/rust-embed
#[derive(RustEmbed)]
#[folder = "src/2022/day3/"]
struct Asset;

pub fn solution() {
    println!("https://adventofcode.com/2022/day/3");

    let file = Asset::get("input.txt").unwrap();

    let vec = file.data.to_owned();
    let res = str::from_utf8(&vec).unwrap();

    let rucksacks = res.lines();

    let mut sum = 0;
    for sack in rucksacks {
        // split lines in half; these are the two compartments
        let (l, r) = sack.split_at(sack.len() / 2);
        // find intersection between the two strings-as-lists
        let lc: HashSet<char> = l.chars().collect();
        let rc: HashSet<char> = r.chars().collect();

        // the intersection appears to be only be a single item.
        let char = lc.intersection(&rc).nth(0).unwrap();

        let priority = ASCII.iter().position(|e| e == char).unwrap();
        sum += priority;
    }

    println!("==> {}", sum) // 8515
}
