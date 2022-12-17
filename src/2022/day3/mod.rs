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
    // clone this so that the value may be used again for part 2
    for sack in rucksacks.clone() {
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

    println!("==> {}", sum); // 8515

    let mut sum = 0;
    // Part 2
    // Chunk the file by 3 rows:
    //
    // > Note: An iterator only provides one element at a time,
    // > whereas a slice is about getting several elements at a time.
    // > - https://stackoverflow.com/a/33189335/9823455
    let rucksacks_vec = rucksacks.map(|line| line.clone()).collect::<Vec<&str>>();
    // ↑ This is how to convert Lines() to Vec<&str>,
    //   so that I can call `.chunks(n)` ↓
    let elf_groups = rucksacks_vec.chunks(3);

    for group in elf_groups {
        // get intersection of the 3 lines in each group
        if let [a, b, c] = *group {
            let ac = a.chars().collect::<HashSet<char>>();
            let bc = b.chars().collect::<HashSet<char>>();
            let cc = c.chars().collect::<HashSet<char>>();

            // I got a compiler error by chaining too many methods here.
            // Needed to use a "let binding" to have temporary values live longer.
            let ac_bc_inter = ac.intersection(&bc).map(|e| *e).collect::<HashSet<char>>();

            // this needs to be mutable for `.nth()` to compile
            let mut common = ac_bc_inter.intersection(&cc);
            let char = common.nth(0).unwrap();

            let priority = ASCII.iter().position(|e| e == char).unwrap();
            sum += priority;
        }
    }

    println!("==> {}", sum);
}
