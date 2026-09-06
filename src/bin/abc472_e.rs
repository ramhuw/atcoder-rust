use std::{fmt::Write, io::Read};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let mut ans = String::new();
    let t: usize = tokens.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let m: usize = tokens.next().unwrap().parse().unwrap();
        let mut g = Graph::new(n);
        for _ in 0..m {
            let a: usize = tokens.next().unwrap().parse().unwrap();
            let b: usize = tokens.next().unwrap().parse().unwrap();
            g.add(a, b);
        }
        let mut search: Vec<(usize, usize, Option<usize>)> = vec![(0, 0, None)];
        let mut found = false;
        while let Some((i, l, p)) = search.pop() {
            if g.nodes[i].nth.is_some() && (l + g.nodes[i].nth.unwrap()) & 1 == 1 {
                found = true;
                write!(ans, "{}\n", l - g.nodes[i].nth.unwrap()).unwrap();
                let mut back_track: Vec<usize> = vec![];
                let mut j = i;
                for x in 1..=(l - g.nodes[i].nth.unwrap()) {
                    for &o in &g.nodes[j].ad {
                        if g.nodes[o].nth.is_some() && g.nodes[o].nth.unwrap() + x == l {
                            j = o;
                            back_track.push(j);
                        }
                    }
                }
                for p in 1..=back_track.len() {
                    write!(ans, "{} ", back_track[back_track.len() - p] + 1).unwrap()
                }
                ans.push('\n');
                break;
            }
            if g.nodes[i].nth.is_none() {
                g.nodes[i].nth = Some(l);
                for &x in &g.nodes[i].ad {
                    if Some(x) != p {
                        search.push((x, l + 1, Some(i)));
                    }
                }
            }
        }
        if !found {
            write!(ans, "-1\n").unwrap();
        }
    }
    println!("{ans}");
}

#[derive(Clone)]
struct Node {
    nth: Option<usize>,
    ad: Vec<usize>,
}
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![
                Node {
                    nth: None,
                    ad: vec![]
                };
                n
            ],
        }
    }
    fn add(&mut self, a: usize, b: usize) {
        self.nodes[a - 1].ad.push(b - 1);
        self.nodes[b - 1].ad.push(a - 1);
    }
}
