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
    let mut s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut x: usize = 0;
    let mut y: usize = 0;

    while s[0] == '.' {
        x += 1;
        for i in 0..n {
            if i + 1 >= n {
                continue;
            }
            if s[i + 1] == '#' {
                s[i] = '#';
            }
        }
    }

    while s[n - 1] == '.' {
        y += 1;
        for i in (0..n).rev() {
            if i == 0 {
                continue;
            }
            if s[i - 1] == '#' {
                s[i] = '#';
            }
        }
    }
    eprintln!("x = {:?}", x);
    eprintln!("y = {:?}", y);
    eprintln!("s = {:?}", s);

    let mut m = 0;
    let mut current = 0;
    for i in 0..n {
        if s[i] == '.' {
            current += 1;
        } else {
            m = max(m, current);
            current = 0;
        }
    }
    y += m;
    format!("{} {}", x, y)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
.#..#",
            "1 1"
        ),
        (
            r"6
..#...",
            "2 3"
        ),
        (
            r"3
###",
            "0 0"
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