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

    let mut r: usize = 0;
    let mut b: usize = 0;
    for _ in 0..n {
        let s: Vec<char> = iterator.next().unwrap().chars().collect();
        for i in 0..n {
            match s[i] {
                'R' => { r += 1 },
                'B' => { b += 1 },
                _ => {},
            }
        }
    }

    if r > b {
        return "TAKAHASHI".to_string();
    }

    if b > r {
        return "AOKI".to_string();
    }

    return "DRAW".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
R.RB
RR.B
BRBB
RRB.",
            "TAKAHASHI"
        ),
        (
            r"2
..
..",
            "DRAW"
        ),
        (
            r"3
BRB
RBR
BRB",
            "AOKI"
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