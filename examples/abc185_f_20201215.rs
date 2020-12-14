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

    let mut st =
        segtree::SegmentTree::new(n, 0, |x, y| x ^ y);

    for i in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        st.update(i, ai);
    }

    let mut result = String::new();
    for _ in 0.. q {
        let t: usize = iterator.next().unwrap().parse().unwrap();
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        if t == 1 {
            st.update(x - 1, st.get(x - 1) ^ y);
        } else {
            let r = st.query(x - 1, y);
            result.push_str(&format!("{}\n", r));
        }
    }
    result
}

#[allow(dead_code)]
mod segtree {
    pub struct SegmentTree<T, F> {
        n: usize,
        dat: Vec<T>,
        initial_value: T,
        op: F,
    }

    impl<T: Copy, F> SegmentTree<T, F>
        where
            F: Fn(T, T) -> T,
    {
        pub fn new(n: usize, initial_value: T, op: F) -> Self {
            let mut m: usize = 1;
            while m < n {
                m *= 2;
            }
            Self {
                n: m,
                dat: vec![initial_value; 2 * m - 1],
                initial_value,
                op,
            }
        }

        pub fn get(&self, i: usize) -> T {
            self.dat[i + self.n - 1]
        }

        pub fn update(&mut self, i: usize, value: T) {
            let mut i = i + self.n - 1;
            self.dat[i] = value;
            while i > 0 {
                i = (i - 1) / 2;
                self.dat[i] = (self.op)(self.dat[i * 2 + 1], self.dat[i * 2 + 2]);
            }
        }

        pub fn query(&self, a: usize, b: usize) -> T {
            self._query(a, b, 0, 0, self.n)
        }

        fn _query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
            if r <= a || b <= l {
                self.initial_value
            } else if a <= l && r <= b {
                self.dat[k]
            } else {
                let vl = self._query(a, b, k * 2 + 1, l, (l + r) / 2);
                let vr = self._query(a, b, k * 2 + 2, (l + r) / 2, r);
                (self.op)(vl, vr)
            }
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
1 2 3
2 1 3
2 2 3
1 2 3
2 2 3",
            "0
1
2"
        ),
        (
            r"10 10
0 5 3 4 7 0 0 0 1 0
1 10 7
2 8 9
2 3 6
2 1 6
2 1 10
1 9 4
1 6 1
1 6 3
1 1 7
2 3 5",
            "1
0
5
3
0"
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