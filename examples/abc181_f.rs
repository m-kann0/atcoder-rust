use std::io::Read;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut points: Vec<(isize, isize)> = Vec::new();
    for _ in 0..n {
        let x: isize = iterator.next().unwrap().parse().unwrap();
        let y: isize = iterator.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    let mut q: BinaryHeap<Reverse<(isize, usize, usize)>> = BinaryHeap::new();
    for i in 0..n {
        let (x1, y1) = points[i];
        for j in 0..n {
            if j <= i {
                continue;
            }
            let (x2, y2) = points[j];
            let d = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
            q.push(Reverse((d, i + 1, j + 1)));
        }
        let d = (100 - y1) * (100 - y1);
        q.push(Reverse((d, 0, i + 1)));
        let d = (-100 - y1) * (-100 - y1);
        q.push(Reverse((d, i + 1, n + 1)));
    }

    let mut dsu = Dsu::new(n + 2);
    while let Some(Reverse((d, u, v))) = q.pop() {
        dsu.merge(u, v);
        if dsu.same(0, n + 1) {
            return format!("{}", (d as f64).sqrt() / 2.0);
        }
    }

    return String::new();
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


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
0 -40
0 40",
            "40"
        ),
        (
            r"4
0 -10
99 10
0 91
99 -91",
            "50.5"
        ),
        (
            r"10
-90 40
20 -30
0 -90
10 -70
80 70
-90 30
-20 -80
10 90
50 30
60 -70",
            "33.541019662496845446"
        ),
        (
            r"10
65 -90
-34 -2
62 99
42 -13
47 -84
84 87
16 -78
56 35
90 8
90 19",
            "35.003571246374276203"
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