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

    let d: usize = iterator.next().unwrap().parse().unwrap();
    let g: usize = iterator.next().unwrap().parse().unwrap();
    let mut p: Vec<usize> = Vec::new();
    let mut c: Vec<usize> = Vec::new();
    for _ in 0..d {
        let pi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        p.push(pi);
        c.push(ci);
    }

    let mut ans: usize = std::usize::MAX;
    for bit in 0..(1 << d) {
        let mut point: usize = 0;
        let mut cost: usize = 0;

        for i in 0..d {
            if bit & 1 << i > 0 {
                point += 100 * (i + 1) * p[i] + c[i];
                cost += p[i];
            }
        }

        if point >= g {
            ans = min(ans, cost);
            continue;
        }

        for i in (0..d).rev() {
            if bit & 1 << i > 0 {
                continue;
            }
            if point + 100 * (i + 1) * (p[i] - 1) < g {
                point += 100 * (i + 1) * (p[i] - 1);
                cost += p[i] - 1;
            } else {
                if (g - point) % (100 * (i + 1)) == 0 {
                    cost += (g - point) / (100 * (i + 1));
                    point += 100 * (i + 1) * ((g - point) / (100 * (i + 1)));
                } else {
                    cost += (g - point) / (100 * (i + 1)) + 1;
                    point += 100 * (i + 1) * ((g - point) / (100 * (i + 1)) + 1);
                }
                break;
            }
        }
        if point >= g {
            ans = min(ans, cost);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 700
3 500
5 800",
            "3"
        ),
        (
            r"2 2000
3 500
5 800",
            "7"
        ),
        (
            r"2 400
3 500
5 800",
            "2"
        ),
        (
            r"5 25000
20 1000
40 1000
50 1000
30 1000
1 1000",
            "66"
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