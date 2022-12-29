pub fn solution() {
    println!("https://adventofcode.com/2022/day/1");
    let res = include_str!("input.txt");

    let mut elves: Vec<u32> = res
        // split by double newlines
        .split("\n\n")
        // ...for each multiline string, call .lines()
        // ...convert each line to integer
        // ...return list of ints
        // ...then call .sum() - "sum values in a list" https://stackoverflow.com/a/23100794/9823455
        .map(
            // "string to int" https://stackoverflow.com/a/27683271/9823455
            |e| e.lines().map(|e| e.parse::<u32>().unwrap()).sum(),
        )
        .collect();

    // todo: explain why what .enumerate() does -> that resulted in `// Some((250, 39436))`
    // let max_calories = elves.iter().max().unwrap();
    // println!("==> {:?}", max_calories); // 72511

    // part 2
    // I needed  to make `elves` mutable so that I could call `sort_by`.
    elves.sort_by(|a, b| b.cmp(a));

    println!("==> {:?}", elves[0]); // 72511

    // grab a slice (start until index 3) of the elves (list of calories)
    println!("==> {:?}", &elves[..3].iter().sum::<u32>()); // 212117
}
