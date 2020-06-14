use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        *counts.entry(ai).or_insert(0) += 1;
        a.push(ai);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        let ai = a[i];

        let mut is_ok = true;
        let mut x = 1;
        while x * x <= ai {
            if ai % x == 0 {
                let y = ai / x;
                if x == ai {
                    if *counts.get(&x).unwrap() >= 2 {
                        is_ok = false;
                    }
                } else {
                    if counts.contains_key(&x) {
                        is_ok = false;
                    }
                }
                if y == ai {
                    if *counts.get(&y).unwrap() >= 2 {
                        is_ok = false;
                    }
                } else {
                    if counts.contains_key(&y) {
                        is_ok = false;
                    }
                }
            }
            x += 1;
        }
        if is_ok {
            ans += 1;
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
24 11 8 3 16",
            "3"
        ),
        (
            r"4
5 5 5 5",
            "0"
        ),
        (
            r"10
33 18 45 28 8 19 89 86 2 4",
            "5"
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