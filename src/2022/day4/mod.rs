pub fn solution() {
    println!("https://adventofcode.com/2022/day/4");

    let res = include_str!("input.txt");

    let assignment_pairs = res.lines();

    let mut contain_ct = 0;
    let mut overlap_ct = 0;

    // clone so that part 2 does not try to use a moved value
    for pair in assignment_pairs {
        let ranges: Vec<&str> = pair.split(",").collect();
        if let [l, r] = ranges[..] {
            let l_bounds: Vec<u32> = l.split("-").map(|e| e.parse::<u32>().unwrap()).collect();
            let r_bounds: Vec<u32> = r.split("-").map(|e| e.parse::<u32>().unwrap()).collect();

            let l_min = l_bounds[0];
            let l_max = l_bounds[1];
            let r_min = r_bounds[0];
            let r_max = r_bounds[1];

            // check if either list contains the other
            let l_in_r = l_min >= r_min && l_max <= r_max;
            let r_in_l = r_min >= l_min && r_max <= l_max;
            if l_in_r || r_in_l {
                contain_ct += 1;
            }

            // check if there is any overlap at all
            let l_over_r = l_min <= r_max && l_max >= r_min;
            let r_over_l = l_max >= r_min && l_min <= r_max;
            if l_over_r || r_over_l {
                overlap_ct += 1;
            }
        }
    }

    // Part 1
    println!("==> {}", contain_ct); // 567

    // Part 2
    println!("==> {}", overlap_ct); // 907
}
