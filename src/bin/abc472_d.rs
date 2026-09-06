use std::{collections::{HashSet, VecDeque}, io::Read};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let h: usize = tokens.next().unwrap().parse().unwrap();
    let w: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let mut grid: Vec<bool> = vec![true; h * w];
    let mut unsafei: HashSet<usize> = HashSet::new();
    let mut unsafej: HashSet<usize> = HashSet::new();
    for i in 0..h {
        let mut line = tokens.next().unwrap().chars();
        for j in 0..w {
            let c = line.next().unwrap();
            if c == '#' {
                grid[f(i, j, w)] = false;
                unsafei.insert(i);
                unsafej.insert(j);
            }
        }
    }
    let mut search = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if !unsafei.contains(&i) && !unsafej.contains(&j) {
                search.push_back((i, j, 0usize));
            }
        }
    }
    let mut reachable: Vec<usize> = vec![usize::MAX; h * w];
    let mut ans: usize = 0;
    while let Some((i, j, l)) = search.pop_front() {
        if !grid[f(i, j, w)] || (reachable[f(i, j, w)] <= l) || l > k {
            continue;
        }
        if reachable[f(i, j, w)] == usize::MAX {
            ans += 1;
        }
        reachable[f(i, j, w)] = l;
        if i > 0 {
            search.push_back((i-1, j, l+1));
        }
        if j > 0 {
            search.push_back((i, j-1, l+1));
        }
        if i < h - 1 {
            search.push_back((i+1, j, l+1));
        }
        if j < w - 1 {
            search.push_back((i, j+1, l+1));
        }
    }
    println!("{}", ans);
}

fn f(i: usize, j: usize, w: usize) -> usize {
    i * w + j
}
