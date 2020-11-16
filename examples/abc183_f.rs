#![allow(non_snake_case)]

use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut counts: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];
    for i in 0..n {
        let c: usize = iterator.next().unwrap().parse().unwrap();
        counts[i].insert(c, 1);
    }

    let mut result = String::new();
    let mut dsu = Dsu::new(n);
    for _ in 0..q {
        let t: usize = iterator.next().unwrap().parse().unwrap();
        if t == 1 {
            let a: usize = iterator.next().unwrap().parse().unwrap();
            let b: usize = iterator.next().unwrap().parse().unwrap();
            let a = a - 1;
            let b = b - 1;
            if dsu.same(a, b) {
                continue;
            }
            let ap = dsu.leader(a);
            let bp = dsu.leader(b);
            let root = dsu.merge(a, b);
            if root == ap {
                let p = take(&mut counts[bp]);
                for (k, v) in p {
                    *counts[root].entry(k).or_insert(0) += v;
                }
            } else {
                let p = take(&mut counts[ap]);
                for (k, v) in p {
                    *counts[root].entry(k).or_insert(0) += v;
                }
            }
        } else {
            let x: usize = iterator.next().unwrap().parse().unwrap();
            let y: usize = iterator.next().unwrap().parse().unwrap();
            let x = x - 1;
            let ans = counts[dsu.leader(x)].get(&y).unwrap_or(&0);
            result.push_str(&format!("{}\n", ans));
        }
    }
    result.trim().to_string()
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        // 0 <= size <= 10^8 is constrained.
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert_eq!(d.same(0, 1), true);
            d.merge(1, 2);
            assert_eq!(d.same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.same(0, 3), false);
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
use dsu::*;
use std::collections::HashMap;
use std::mem::take;

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 5
1 2 3 2 1
1 1 2
1 2 5
2 1 1
1 3 4
2 3 4",
            "2
0"
        ),
        (
            r"5 4
2 2 2 2 2
1 1 2
1 1 3
1 2 3
2 2 2",
            "3"
        ),
        (
            r"12 9
1 2 3 1 2 3 1 2 3 1 2 3
1 1 2
1 3 4
1 5 6
1 7 8
2 2 1
1 9 10
2 5 6
1 4 8
2 6 1",
            "1
0
0"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
            println!("OK");
        } else {
            println!("NG");
            println!("    Expected: {}", expected);
            println!("    Actual  : {}", actual);

            all_ok = false;
        }
    }

    assert_eq!(all_ok, true);
}