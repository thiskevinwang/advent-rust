use rust_embed::RustEmbed;
use std::str;

// https://crates.io/crates/rust-embed
#[derive(RustEmbed)]
#[folder = "src/2022/day4/"]
struct Asset;
pub fn solution() {
    println!("https://adventofcode.com/2022/day/4");

    let file = Asset::get("input.txt").unwrap();

    let vec = file.data.to_owned();
    let res = str::from_utf8(&vec).unwrap();

    let assignment_pairs = res.lines();

    let mut overlap_ct = 0;

    for pair in assignment_pairs {
        let ranges: Vec<&str> = pair.split(",").collect();
        if let [l, r] = ranges[..] {
            let l_bounds = l.split("-").collect::<Vec<&str>>();
            let r_bounds = r.split("-").collect::<Vec<&str>>();

            let l_min = l_bounds[0].parse::<u32>().unwrap();
            let l_max = l_bounds[1].parse::<u32>().unwrap();
            let r_min = r_bounds[0].parse::<u32>().unwrap();
            let r_max = r_bounds[1].parse::<u32>().unwrap();

            if (l_min >= r_min && l_max <= r_max) || (r_min >= l_min && r_max <= l_max) {
                overlap_ct += 1;
            }
        }
    }

    // Part 1
    println!("==> {}", overlap_ct);

    // Part 2
    // println!("==> {}" /* */);
}
