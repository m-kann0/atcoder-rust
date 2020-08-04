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
    let mut f: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let mut fi: Vec<char> = Vec::new();
        for _ in 0..10 {
            fi.push(iterator.next().unwrap().parse().unwrap());
        }
        f.push(fi);
    }
    let mut p: Vec<Vec<isize>> = Vec::new();
    for _ in 0..n {
        let mut pi: Vec<isize> = Vec::new();
        for _ in 0..11 {
            pi.push(iterator.next().unwrap().parse().unwrap());
        }
        p.push(pi);
    }

    let mut ans: isize = std::isize::MIN;
    for i in 1..(1<<10) {
        let q: Vec<char> = binary_string(i, 10).chars().collect();
        let mut current = 0;
        for j in 0..n {
            let mut count = 0;
            for k in 0..10 {
                if q[k] == '1' && f[j][k] == '1' {
                    count += 1;
                }
            }
            current += p[j][count]
        }
        ans = max(ans, current);
    }
    ans.to_string()
}

fn binary_string(n: usize, keta: usize) -> String {
    return format!("{:0>1$b}", n, keta);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1
1 1 0 1 0 0 0 1 0 1
3 4 5 6 7 8 9 -2 -3 4 -2",
            "8"
        ),
        (
            r"2
1 1 1 1 1 0 0 0 0 0
0 0 0 0 0 1 1 1 1 1
0 -2 -2 -2 -2 -2 -1 -1 -1 -1 -1
0 -2 -2 -2 -2 -2 -1 -1 -1 -1 -1",
            "-2"
        ),
        (
            r"3
1 1 1 1 1 1 0 0 1 1
0 1 0 1 1 1 1 0 1 0
1 0 1 1 0 1 0 1 0 1
-8 6 -2 -8 -8 4 8 7 -6 2 2
-9 2 0 1 7 -5 0 -2 -6 5 5
6 -6 7 -9 6 -5 8 0 -9 -7 -7",
            "23"
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