use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let k: isize = iterator.next().unwrap().parse().unwrap();

    let mut ans: isize = 0;
    for x in 2..=(2*n) {
        let y = x - k;
        if y < 2 {
            continue;
        }
        let s = if x <= n + 1 {
            x - 1
        } else {
            (x - 1) - (x - 1 - n) * 2
        };
        let t = if y <= n + 1 {
            y - 1
        } else {
            (y - 1) - (y - 1 - n) * 2
        };
        if s < 0 || t < 0 {
            continue;
        }
        ans += s * t;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 1",
            "4"
        ),
        (
            r"2525 -425",
            "10314607400"
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