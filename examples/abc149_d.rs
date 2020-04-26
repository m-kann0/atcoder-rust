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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let r: usize = iterator.next().unwrap().parse().unwrap();
    let s: usize = iterator.next().unwrap().parse().unwrap();
    let p: usize = iterator.next().unwrap().parse().unwrap();

    let mut t: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut ans = 0;

    for i in 0..n {
        if i >= k && t[i] == t[i - k] {
            t[i] = 'x';
        } else {
            let hand = t[i];
            if hand == 'r' {
                ans += p;
            } else if hand == 's' {
                ans += r;
            } else if hand == 'p' {
                ans += s;
            }
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2
8 7 6
rsrpr",
            "27"
        ),
        (
            r"7 1
100 10 1
ssssppr",
            "211"
        ),
        (
            r"30 5
325 234 123
rspsspspsrpspsppprpsprpssprpsr",
            "4996"
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