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
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut uf = union_find::UnionFind::new(n);
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        uf.unite(ai - 1, bi - 1);
    }

    let ans = (0..n)
        .map(|i| uf.size(i))
        .max()
        .unwrap();
    ans.to_string()
}

#[allow(dead_code)]
mod union_find {
    pub struct UnionFind {
        parent: Vec<usize>,
        sizes: Vec<usize>,
        len: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind {
                parent: (0..n).map(|i| i).collect(),
                sizes: vec![1; n],
                len: n,
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let px = self.parent[x];
                self.parent[x] = self.find(px);
                self.parent[x]
            }
        }

        pub fn unite(&mut self, x: usize, y: usize) {
            let px = self.find(x);
            let py = self.find(y);

            if px == py {
                return;
            }

            let (large, small) = if self.sizes[px] < self.sizes[py] {
                (py, px)
            } else {
                (px, py)
            };

            self.parent[small] = large;
            self.sizes[large] += self.sizes[small];
            self.sizes[small] = 0;
            self.len -= 1;
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }

        pub fn size(&mut self, x: usize) -> usize {
            let px = self.find(x);
            self.sizes[px]
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }
}


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
1 2
3 4
5 1",
            "3"
        ),
        (
            r"4 10
1 2
2 1
1 2
2 1
1 2
1 3
1 4
2 3
2 4
3 4",
            "4"
        ),
        (
            r"10 4
3 1
4 1
5 9
2 6",
            "3"
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