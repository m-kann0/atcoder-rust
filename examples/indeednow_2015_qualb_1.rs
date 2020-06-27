use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x1: isize = iterator.next().unwrap().parse().unwrap();
    let y1: isize = iterator.next().unwrap().parse().unwrap();
    let x2: isize = iterator.next().unwrap().parse().unwrap();
    let y2: isize = iterator.next().unwrap().parse().unwrap();

    let ans = (x1 - x2).abs() + (y1 - y2).abs() + 1;
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
2 5",
            "4"
        ),
        (
            r"1 2
1 1",
            "2"
        ),
        (
            r"20 40
32 64",
            "37"
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