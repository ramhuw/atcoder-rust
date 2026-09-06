use std::{fmt::Write, io::Read};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    if n == 1 {
        println!("1");
        return;
    }
    let q = tokens.next().unwrap();
    let mut current = tokens.next().unwrap() - 1;
    let mut list = List {
        head: current,
        nodes: vec![
            Node {
                prev: None,
                next: None
            };
            n
        ],
        tail: 0,
    };
    for _ in 1..n {
        let p = tokens.next().unwrap() - 1;
        list.nodes[current].next = Some(p);
        list.nodes[p].prev = Some(current);
        current = p;
    }
    list.tail = current;
    for _ in 0..q {
        let a = tokens.next().unwrap() - 1;
        let left_maybe = list.nodes[a].prev;
        let right_maybe = list.nodes[a].next;
        if let Some(left) = left_maybe {
            list.nodes[left].next = right_maybe;
        } else {
            if let Some(right) = right_maybe {
                list.head = right
            }
        }
        if let Some(right) = right_maybe {
            list.nodes[right].prev = left_maybe;
        } else {
            if let Some(left) = left_maybe {
                list.tail = left
            }
        }

        list.nodes[a].prev = Some(list.tail);
        list.nodes[list.tail].next = Some(a);
        list.nodes[a].next = None;
        list.tail = a;
    }
    let mut ans: String = String::new();
    let mut current = list.head;
    for i in 0..n {
        write!(ans, "{} ", current + 1).unwrap();
        if i == n - 1 {
            break;
        }
        current = list.nodes[current].next.unwrap();
    }
    println!("{ans}");
}

#[derive(Copy, Clone)]
struct Node {
    prev: Option<usize>,
    next: Option<usize>,
}

struct List {
    head: usize,
    tail: usize,
    nodes: Vec<Node>,
}
