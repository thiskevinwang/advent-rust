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
            let l_bounds: Vec<u32> = l.split("-").map(|e| e.parse::<u32>().unwrap()).collect();
            let r_bounds: Vec<u32> = r.split("-").map(|e| e.parse::<u32>().unwrap()).collect();

            let l_min = l_bounds[0];
            let l_max = l_bounds[1];
            let r_min = r_bounds[0];
            let r_max = r_bounds[1];

            let l_in_r = l_min >= r_min && l_max <= r_max;
            let r_in_l = r_min >= l_min && r_max <= l_max;
            if l_in_r || r_in_l {
                overlap_ct += 1;
            }
        }
    }

    // Part 1
    println!("==> {}", overlap_ct); // 567

    // Part 2
    // println!("==> {}" /* */);
}
