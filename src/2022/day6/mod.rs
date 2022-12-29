use std::collections::HashSet;
use std::hash::Hash;
pub fn solution() {
    println!("https://adventofcode.com/2022/day/6");
    let res = include_str!("input.txt");
    // convert input into a iterator of chars
    let chars = res.chars().into_iter();

    // Part 1
    for (i, _) in chars.clone().enumerate() {
        // guard against lower overflow
        if i < 3 {
            continue;
        }
        let vec = chars.clone().collect::<Vec<char>>();

        let start_of_packet = &vec[i - 3..=i];
        if has_unique_elements(start_of_packet) {
            // Read the prompt carefully; Beware of the off-by-one pitfall.
            println!("==> {}", i + 1); // 1912
            break;
        }
    }

    // Part 2
    for (i, _) in chars.clone().enumerate() {
        // guard against lower overflow
        if i < 13 {
            continue;
        }
        let vec = chars.clone().collect::<Vec<char>>();

        let start_of_message = &vec[i - 13..=i];
        if has_unique_elements(start_of_message) {
            println!("==> {}", i + 1); // 2122
            break;
        }
    }
}

/// Check if a slice has unique elements
/// https://stackoverflow.com/a/46767732/9823455
fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
