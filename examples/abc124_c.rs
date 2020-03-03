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

    let s: &str = iterator.next().unwrap();

    let mut s1 = String::new();
    let mut s2 = String::new();
    for i in 0..s.len() {
        s1.push(if i % 2 == 0 { '0' } else { '1' });
        s2.push(if i % 2 == 0 { '1' } else { '0' });
    }

    let mut count1 = 0;
    let mut count2 = 0;

    let mut it = s.chars();
    let mut it1 = s1.chars();
    let mut it2 = s2.chars();
    for _i in 0..s.len() {
        let c = it.next().unwrap();
        let c1 = it1.next().unwrap();
        let c2 = it2.next().unwrap();
        if c != c1 {
            count1 += 1;
        }
        if c != c2 {
            count2 += 1;
        }
    }

    return format!("{}", min(count1, count2));
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"000",
            "1"
        ),
        (
            r"10010010",
            "3"
        ),
        (
            r"0",
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