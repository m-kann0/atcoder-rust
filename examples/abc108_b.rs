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

    let dx = x2 - x1;
    let dy = y2 - y1;

    let x3 = x2 - dy;
    let y3 = y2 + dx;
    let x4 = x3 - dx;
    let y4 = y3 - dy;

    return format!("{} {} {} {}", x3, y3, x4, y4);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"0 0 0 1",
            "-1 1 -1 0"
        ),
        (
            r"2 3 6 6",
            "3 10 -1 7"
        ),
        (
            r"31 -41 -59 26",
            "-126 -64 -36 -131"
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