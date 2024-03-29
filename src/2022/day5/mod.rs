use regex::Regex;
use std::{collections::HashMap, str};

pub fn solution() {
    println!("https://adventofcode.com/2022/day/5");
    let res = include_str!("input.txt");

    if let [stack_str, procedures] = res.split("\n\n").collect::<Vec<&str>>()[..] {
        // Stacks are built bottom up from lines 8 to 1
        // Map keys at line 9
        // Procedures start at line 11

        // Get indices of columns
        // 1 @ 1
        // 2 @ 5
        // 3 @ 9
        // 4 @ 13
        // 5 @ 17
        // 6 @ 21
        // 7 @ 25
        // 8 @ 29
        // 9 @ 33

        // Note: declare map as mutable because we'll call `.get_mut()`
        // https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_mut
        let mut map: HashMap<usize, Vec<char>> = HashMap::from([
            (1, vec![]),
            (2, vec![]),
            (3, vec![]),
            (4, vec![]),
            (5, vec![]),
            (6, vec![]),
            (7, vec![]),
            (8, vec![]),
            (9, vec![]),
        ]);

        // use this cloned map for part 2
        let mut map2 = map.clone();

        // Build our hashmap of stacks
        // - reverse the lines so we can build each stack upwards
        let stack_lines = stack_str.lines().rev();
        for (i, line) in stack_lines.enumerate() {
            if i == 0 {
                // Disregard this line; It holds keys.
                continue;
            }
            for (i, col_idx) in vec![1, 5, 9, 13, 17, 21, 25, 29, 33].iter().enumerate() {
                // Types needed to access vector by index? -> Vec<char>[usize]
                // And you can dereference `&i32` and cast to `usize`
                let char = line.chars().collect::<Vec<char>>()[*col_idx as usize];

                // if there is indeed a char at this index in input ([ ]),
                // push it onto our stack, located at the designated key
                if char != ' ' {
                    map.get_mut(&(i + 1)).unwrap().push(char);
                    // for part 2...
                    map2.get_mut(&(i + 1)).unwrap().push(char);
                }
            }
        }

        let re = Regex::new(r"^move (\d+) from (\d+) to (\d)$").unwrap();

        let procs = procedures.lines();

        for proc in procs {
            for cap in re.captures_iter(proc) {
                let source = &cap[2].parse::<usize>().unwrap();
                let target = &cap[3].parse::<usize>().unwrap();

                // pop these many from the stack at [source] onto the stack at [target]
                let mv = cap[1].parse::<usize>().unwrap();

                // For the number of "crates" the elves want to move...
                // (Crates are moved in FILO — first in, last out)
                for _ in 0..mv {
                    // NOTE: cannot write code like this because
                    // > cannot borrow `map` as mutable more than once at a time
                    // > second mutable borrow occurs here
                    // -----
                    // let src = map.get_mut(source).unwrap();
                    // let trg = map.get_mut(target).unwrap();
                    // trg.push(src.pop().unwrap());
                    // -----
                    // BUT: you can modify it slightly to compile
                    let el = map.get_mut(source).unwrap().pop().unwrap();
                    map.get_mut(target).unwrap().push(el)
                }

                // Part 2
                {
                    // how to pop multiple elements
                    // https://stackoverflow.com/a/28952552/9823455
                    let source = map2.get_mut(source).unwrap();
                    let final_length = source.len().saturating_sub(mv);
                    let tail = source.split_off(final_length);

                    // how to concat vectors
                    // https://stackoverflow.com/a/40795247/9823455
                    map2.get_mut(target).unwrap().extend(tail);
                }
            }
        }

        // Because hashmap keys are unsorted, I'll use a fixed slice (0..8)
        // and use its indices, adding 1 to each, to access the hashmaps keys
        // in "sorted" manner.
        //
        // For each index, I grab the the hashmap's value (Vec<char>) and
        // get the last item, and fold (most similar to JS reduce) every last item into a string.
        let crates = [0; 9]
            .iter()
            .enumerate()
            .map(|(i, _)| return map.get(&(i + 1)).unwrap().last().unwrap())
            // how to concate a `char` to a `String`
            // https://stackoverflow.com/a/37889710/9823455
            .fold("".to_string(), |mut acc, ch| {
                acc.push(*ch);
                return acc;
            });

        // Part 1
        println!("==> {}", crates); // PTWLTDSJV

        // Part 2 — now crates are moved in FIFO (first in, first out), instead of FILO previously
        let crates2 = [0; 9]
            .iter()
            .enumerate()
            .map(|(i, _)| return map2.get(&(i + 1)).unwrap().last().unwrap())
            // how to concate a `char` to a `String`
            // https://stackoverflow.com/a/37889710/9823455
            .fold("".to_string(), |mut acc, ch| {
                acc.push(*ch);
                return acc;
            });

        println!("==> {}", crates2); // WZMFVGGZP
    }
}
