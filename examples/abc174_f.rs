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

    let mut p: Vec<(isize, isize, isize)> = Vec::new();
    let mut prev: Vec<Option<isize>> = vec![None; n + 1];
    for y in 0..n {
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        if let Some(x) = prev[ci] {
            p.push((-x, y as isize, -1_isize));
        }
        prev[ci] = Some(y as isize);
    }
    for i in 0..q {
        let l: isize = iterator.next().unwrap().parse().unwrap();
        let r: isize = iterator.next().unwrap().parse().unwrap();
        let l = l - 1;
        let r = r - 1;
        p.push((-l, r, i as isize));
    }

    p.sort();
    let p: Vec<(isize, isize, isize)> = p.iter().map(|t| (-t.0, t.1, t.2)).collect();

    // eprintln!("p = {:?}", p);

    let mut result = vec![0; q];
    let mut bit = FenwickTree::new(n + 5, 0_usize);
    for &(x, y, i) in p.iter() {
        if i == -1 {
            bit.add(y as usize, 1);
        } else {
            let now = bit.accum(y as usize + 1);
            result[i as usize] = y - x + 1 - now as isize;
        }
    }

    let mut ans = String::new();
    for i in 0..q {
        ans.push_str(&format!("{}\n", result[i]));
    }
    ans
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod fenwicktree {
    // Reference: https://en.wikipedia.org/wiki/Fenwick_tree
    pub struct FenwickTree<T> {
        n: usize,
        ary: Vec<T>,
        e: T,
    }

    impl<T: Clone + std::ops::AddAssign<T>> FenwickTree<T> {
        pub fn new(n: usize, e: T) -> Self {
            FenwickTree {
                n,
                ary: vec![e.clone(); n],
                e,
            }
        }
        pub fn accum(&self, mut idx: usize) -> T {
            let mut sum = self.e.clone();
            while idx > 0 {
                sum += self.ary[idx - 1].clone();
                idx &= idx - 1;
            }
            sum
        }
        /// performs data[idx] += val;
        pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
            where
                T: std::ops::AddAssign<U>,
        {
            let n = self.n;
            idx += 1;
            while idx <= n {
                self.ary[idx - 1] += val.clone();
                idx += idx & idx.wrapping_neg();
            }
        }
        /// Returns data[l] + ... + data[r - 1].
        pub fn sum(&self, l: usize, r: usize) -> T
            where
                T: std::ops::Sub<Output = T>,
        {
            self.accum(r) - self.accum(l)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fenwick_tree_works() {
            let mut bit = FenwickTree::new(5, 0i64);
            // [1, 2, 3, 4, 5]
            for i in 0..5 {
                bit.add(i, i as i64 + 1);
            }
            assert_eq!(bit.sum(0, 5), 15);
            assert_eq!(bit.sum(0, 4), 10);
            assert_eq!(bit.sum(1, 3), 5);
        }
    }
}
use fenwicktree::*;

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3
1 2 1 3
1 3
2 4
3 3",
            "2
3
1"
        ),
        (
            r"10 10
2 5 6 5 2 1 7 9 7 2
5 5
2 4
6 7
2 2
7 8
7 9
1 8
6 9
8 10
6 8",
            "1
2
2
1
2
2
6
3
3
3"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected.trim() == actual.trim() {
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