use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut d: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for _ in 0..n {
            d[i].push(iterator.next().unwrap().parse().unwrap());
        }
    }
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let p: Vec<usize> = (0..q).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut s: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] + d[i][j] - s[i][j];
        }
    }

    let mut v: Vec<usize> = vec![0; n * n + 1];
    for il in 0..n {
        for iu in (il + 1)..(n + 1) {
            for jl in 0..n {
                for ju in (jl + 1)..(n + 1) {
                    let area = (iu - il) * (ju - jl);
                    v[area] = max(
                        v[area],
                        s[iu][ju] + s[il][jl] - s[il][ju] - s[iu][jl]
                    );
                }
            }
        }
    }
    for i in 1.. {
        if i > n * n {
            break;
        }
        v[i] = max(v[i], v[i - 1]);
    }

    let mut result = String::new();
    for i in 0..q {
        result.push_str(&format!("{}\n", v[p[i]]));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3 2 1
2 2 1
1 1 1
3
1
4
9",
            r"3
9
14"
        ),
        (
            r"3
1 1 1
1 1 1
9 9 9
1
4",
            r"27"
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