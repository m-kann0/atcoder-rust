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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let s: &str = iterator.next().unwrap();

    let mut ans: usize = n;
    let buttons = vec!['A', 'B', 'X', 'Y'];
    for i in 0..4 {
        for j in 0..4 {
            for p in 0..4 {
                for q in 0..4 {
                    let l = &format!("{}{}", buttons[i], buttons[j]);
                    let r = &format!("{}{}", buttons[p], buttons[q]);

                    let len1 = s.replace(l, "L").replace(r, "R").len();
                    let len2 = s.replace(r, "R").replace(l, "L").len();

                    ans = min(ans, min(len1, len2));
                }
            }
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
ABXY",
            r"2"
        ),
        (
            r"13
ABABABABXBXBX",
            r"7"
        ),
        (
            r"8
AABBAABB",
            r"4"
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