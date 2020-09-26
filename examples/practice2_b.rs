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

    let mut ft = FenwickTree::new(n, 0_usize);

    for i in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        ft.add(i, ai);
    }

    let mut result = String::new();
    for _ in 0..q {
        let query_type: usize = iterator.next().unwrap().parse().unwrap();
        if query_type == 0 {
            let p: usize = iterator.next().unwrap().parse().unwrap();
            let x: usize = iterator.next().unwrap().parse().unwrap();
            ft.add(p, x);
        } else {
            let l: usize = iterator.next().unwrap().parse().unwrap();
            let r: usize = iterator.next().unwrap().parse().unwrap();
            result.push_str(&format!("{}\n", ft.sum(l, r)));
        }
    }
    result.trim().to_string()
}

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


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 5
1 2 3 4 5
1 0 5
1 2 4
0 3 10
1 0 5
1 0 3",
            "15
7
25
6"
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