use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: isize = iterator.next().unwrap().parse().unwrap();
    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();

    if a >= b || (b - a) < 2 {
        return (k + 1).to_string();
    }

    if a >= k {
        return (k + 1).to_string();
    }

    let ans = a + (k - a + 1) / 2 * (b - a) + (k - a + 1) % 2;

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2 6",
            "7"
        ),
        (
            r"7 3 4",
            "8"
        ),
        (
            r"314159265 35897932 384626433",
            "48518828981938099"
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