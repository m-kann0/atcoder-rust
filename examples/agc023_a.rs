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
    let a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut s: Vec<isize> = vec![0; n + 1];
    for i in 1..(n + 1) {
        s[i] = s[i - 1] + a[i - 1];
    }

    let mut counts = std::collections::HashMap::new();
    for si in s {
        *counts.entry(si).or_insert(0_usize) += 1;
    }

    let mut ans: usize = 0;
    for (_, count) in counts {
        if count >= 2 {
            ans += count * (count - 1) / 2;
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
1 3 -4 2 2 -2",
            "3"
        ),
        (
            r"7
1 -1 1 -1 1 -1 1",
            "12"
        ),
        (
            r"5
1 -2 3 -4 5",
            "0"
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