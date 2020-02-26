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

    let l: u64 = iterator.next().unwrap().parse().unwrap();
    let r: u64 = iterator.next().unwrap().parse().unwrap();

    return if (r - l) >= 2019 {
        String::from("0")
    } else {
        let mut ans: u64 = 2019;
        for i in l..r {
            for j in (l + 1)..(r + 1) {
                ans = min(ans, (i % 2019) * (j % 2019) % 2019);
            }
        }
        ans.to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2020 2040",
            "2"
        ),
        (
            r"4 5",
            "20"
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