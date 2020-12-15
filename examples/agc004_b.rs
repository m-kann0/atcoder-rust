#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans = std::usize::MAX;
    let mut f = vec![vec![std::usize::MAX; n]; n];
    for k in 0..n {
        let mut now = k * x;
        for i in 0..n {
            f[i][k] = min(f[i][(n + k - 1) % n], a[(n + i - k) % n]);
            now += f[i][k];
        }
        ans = min(ans, now);
    }
    ans.to_string()
}

// fn solve(input: &str) -> String {
//     let mut iterator = input.split_whitespace();
//
//     let n: usize = iterator.next().unwrap().parse().unwrap();
//     let x: usize = iterator.next().unwrap().parse().unwrap();
//     let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
//
//     let mut ans = std::usize::MAX;
//     let mut st = segtree::SegmentTree::new(
//         n,
//         std::usize::MAX,
//         |x, y| min(x, y),
//     );
//     for i in 0..n {
//         st.update(i, a[i]);
//     }
//     for k in 0..n {
//         let mut now = k * x;
//         for i in 0..n {
//             let r = i;
//             let l = (n + i - k) % n;
//             if l <= r {
//                 now += st.query(l, r + 1);
//             } else {
//                 now += min(
//                     st.query(0, r + 1),
//                     st.query(l, n)
//                 );
//             }
//         }
//         ans = min(ans, now);
//     }
//     ans.to_string()
// }
//
// #[allow(dead_code)]
// mod segtree {
//     pub struct SegmentTree<T, F> {
//         n: usize,
//         dat: Vec<T>,
//         initial_value: T,
//         op: F,
//     }
//
//     impl<T: Copy, F> SegmentTree<T, F>
//         where
//             F: Fn(T, T) -> T,
//     {
//         pub fn new(n: usize, initial_value: T, op: F) -> Self {
//             let mut m: usize = 1;
//             while m < n {
//                 m *= 2;
//             }
//             Self {
//                 n: m,
//                 dat: vec![initial_value; 2 * m - 1],
//                 initial_value,
//                 op,
//             }
//         }
//
//         pub fn get(&self, i: usize) -> T {
//             self.dat[i + self.n - 1]
//         }
//
//         pub fn update(&mut self, i: usize, value: T) {
//             let mut i = i + self.n - 1;
//             self.dat[i] = value;
//             while i > 0 {
//                 i = (i - 1) / 2;
//                 self.dat[i] = (self.op)(self.dat[i * 2 + 1], self.dat[i * 2 + 2]);
//             }
//         }
//
//         pub fn query(&self, a: usize, b: usize) -> T {
//             self._query(a, b, 0, 0, self.n)
//         }
//
//         fn _query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
//             if r <= a || b <= l {
//                 self.initial_value
//             } else if a <= l && r <= b {
//                 self.dat[k]
//             } else {
//                 let vl = self._query(a, b, k * 2 + 1, l, (l + r) / 2);
//                 let vr = self._query(a, b, k * 2 + 2, (l + r) / 2, r);
//                 (self.op)(vl, vr)
//             }
//         }
//     }
// }

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 10
1 100",
            "12"
        ),
        (
            r"3 10
100 1 100",
            "23"
        ),
        (
            r"4 10
1 2 3 4",
            "10"
        ),
        (
            r"4 10
1 100 1 100",
            "14"
        ),
        (
            r"4 10
100 1 100 1",
            "14"
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