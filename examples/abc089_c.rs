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

    let march: Vec<char> = "MARCH".chars().collect();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut counts: HashMap<char, usize> = HashMap::new();
    for _ in 0..n {
        let first_char = iterator.next().unwrap().chars().next().unwrap();
        if march.contains(&first_char) {
            *counts.entry(first_char).or_insert(0) += 1;
        }
    }

    let zero = 0;
    let mut ans = 0;
    for i in 0..3 {
        for j in (i + 1)..4 {
            for k in (j + 1)..5 {
                let ci = counts.get(&march[i]).unwrap_or(&zero);
                let cj = counts.get(&march[j]).unwrap_or(&zero);
                let ck = counts.get(&march[k]).unwrap_or(&zero);
                ans += *ci * *cj * *ck;
            }
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
MASHIKE
RUMOI
OBIRA
HABORO
HOROKANAI",
            "2"
        ),
        (
            r"4
ZZ
ZZZ
Z
ZZZZZZZZZZ",
            "0"
        ),
        (
            r"5
CHOKUDAI
RNG
MAKOTO
AOKI
RINGO",
            "7"
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