use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let c11: isize = iterator.next().unwrap().parse().unwrap();
    let c12: isize = iterator.next().unwrap().parse().unwrap();
    let c13: isize = iterator.next().unwrap().parse().unwrap();
    let c21: isize = iterator.next().unwrap().parse().unwrap();
    let c22: isize = iterator.next().unwrap().parse().unwrap();
    let c23: isize = iterator.next().unwrap().parse().unwrap();
    let c31: isize = iterator.next().unwrap().parse().unwrap();
    let c32: isize = iterator.next().unwrap().parse().unwrap();
    let c33: isize = iterator.next().unwrap().parse().unwrap();

    let a1: isize = 0;
    let b1: isize = c11 - a1;
    let b2: isize = c12 - a1;
    let b3: isize = c13 - a1;

    if (c21 - b1) == (c22 - b2) && (c22 - b2) == (c23 - b3)
        && (c31 - b1) == (c32 - b2) && (c32 - b2) == (c33 - b3) {
        return String::from("Yes");
    }

    return String::from("No");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 0 1
2 1 2
1 0 1",
            "Yes"
        ),
        (
            r"2 2 2
2 1 2
2 2 2",
            "No"
        ),
        (
            r"0 8 8
0 8 8
0 8 8",
            "Yes"
        ),
        (
            r"1 8 6
2 9 7
0 7 7",
            "No"
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