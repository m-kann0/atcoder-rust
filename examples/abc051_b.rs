use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: i64 = iterator.next().unwrap().parse().unwrap();
    let s: i64 = iterator.next().unwrap().parse().unwrap();

    let mut count: u64 = 0;
    for x in 0..(k + 1) {
        for y in 0..(k + 1) {
            let z = s - x - y;
            if z >= 0 && z <= k {
                count += 1;
            }
        }
    }
    return format!("{}", count);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2",
            "6"
        ),
        (
            r"5 15",
            "1"
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