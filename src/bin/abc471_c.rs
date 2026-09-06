use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let mut table = vec![];
    let mut right: usize = 0;
    let n: usize = tokens.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let a: i32 = tokens.next().unwrap().parse().unwrap();
        if a < 0 {
            right += 1;
        }
        table.push(a);
    }
    table.sort();
    let mut seg_tree = SegTree::new(vec![1; n]);
    let mut ans: usize = 0;
    let mut current_id: i32 = 0;
    if right == n {
        right -= 1;
    }
    for i in (0..n).rev() {
        let right_iid = seg_tree.nth(right);
        let right_id = table[right_iid];
        if right_id > current_id && right > 0 {
            let left = right - 1;
            let left_iid = seg_tree.nth(left);
            let left_id = table[left_iid];
            if current_id - left_id <= right_id - current_id {
                right -= 1;
                ans += (current_id - left_id) as usize;
                seg_tree.update(left_iid, 0);
                current_id = left_id;
                continue;
            }
        }
        ans += (right_id - current_id).abs() as usize;
        seg_tree.update(right_iid, 0);
        current_id = right_id;
        if right == i && right > 0 {
            right -= 1
        }
    }
    println!("{ans}");
}

use std::ops::Add;
struct SegTree {
    tree: Vec<Vec<usize>>,
}
impl SegTree {
    fn new(arr: Vec<usize>) -> Self {
        let n = arr.len();
        let mut tree = vec![arr];
        for i in 1.. {
            let l = 1 << i;
            if l > n {
                break;
            }
            let line: Vec<usize> = (0..)
                .take_while(|j| l * j + l <= n)
                .map(|j| tree[i - 1][2 * j].add(tree[i - 1][2 * j + 1]))
                .collect();

            tree.push(line);
        }
        Self { tree }
    }

    fn update(&mut self, i: usize, a: usize) {
        let mut i = i;
        self.tree[0][i] = a;
        for j in 1..self.tree.len() {
            i /= 2;
            if i >= self.tree[j].len() {
                break;
            }

            self.tree[j][i] = self.tree[j - 1][2 * i].add(self.tree[j - 1][2 * i + 1]);
        }
    }

    fn nth(&self, n: usize) -> usize {
        let mut k = 0;
        let mut n = n;
        for i in (0..self.tree.len()).rev() {
            if k >= self.tree[i].len() {
                k *= 2;
            } else if self.tree[i][k] <= n {
                n -= self.tree[i][k];
                k = (k + 1) * 2;
            } else {
                k = k * 2;
            }
        }
        k / 2
    }
}
