pub fn solution() {
    println!("https://adventofcode.com/2022/day/8");

    let res = include_str!("input.txt");

    let grid: Vec<Vec<u32>> = res
        .lines()
        .into_iter()
        .map(|e| {
            let res = e
                .chars()
                .into_iter()
                .map(|f| f.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            res
        })
        .collect();

    // assumes the trees come as a rectangle or square
    let mut grid_clone: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    let h = grid.len();
    let w = grid[0].len();

    // Part 1a: for each row, left to right
    for i in 0..h {
        let mut max_height = -1;
        for j in 0..w {
            let tree_height = grid[i][j] as i32;
            if tree_height > max_height {
                grid_clone[i][j] = true;
                max_height = tree_height;
            }
        }
    }
    // Part 1b: for each row, right to left
    for i in 0..h {
        let mut max_height = -1;
        for j in (0..w).rev() {
            let tree_height = grid[i][j] as i32;
            if tree_height > max_height {
                grid_clone[i][j] = true;
                max_height = tree_height;
            }
        }
    }
    // Part 1c: for each column, top to bottom
    for j in 0..w {
        let mut max_height = -1;
        for i in 0..h {
            let tree_height = grid[i][j] as i32;
            if tree_height > max_height {
                grid_clone[i][j] = true;
                max_height = tree_height;
            }
        }
    }
    // Part 1d: for each column, bottom to top
    for j in 0..w {
        let mut max_height = -1;
        for i in (0..h).rev() {
            let tree_height = grid[i][j] as i32;
            if tree_height > max_height {
                grid_clone[i][j] = true;
                max_height = tree_height;
            }
        }
    }

    let visible_tree_count = grid_clone.iter().fold(0, |acc, next| {
        let row_sum = next.iter().fold(0, |sum, &tree| {
            if tree == true {
                return sum + 1;
            } else {
                return sum;
            }
        });
        acc + row_sum
    });

    println!("==> {}", visible_tree_count);
    // for row in 0..grid_clone.len() - 1 {
    //     println!(
    //         "{}",
    //         grid_clone[row]
    //             .iter()
    //             .fold(String::from(""), |mut acc, &next| {
    //                 if next == true {
    //                     acc.push('∆');
    //                 } else {
    //                     acc.push('.');
    //                 }
    //                 acc
    //             }),
    //     )
    // }
    // Part 2
    println!("==> {}", "TODO");
}
