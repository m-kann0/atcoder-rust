use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: isize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut p: HashSet<isize> = HashSet::new();
    for _ in 0..n {
        p.insert(iterator.next().unwrap().parse().unwrap());
    }

    for i in 0..200 {
        if !p.contains(&(x - i)) {
            return format!("{}", x - i);
        }
        if !p.contains(&(x + i)) {
            return format!("{}", x + i);
        }
    }

    return String::new();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6 5
4 7 10 6 5",
            "8"
        ),
        (
            r"10 5
4 7 10 6 5",
            "9"
        ),
        (
            r"100 0",
            "100"
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