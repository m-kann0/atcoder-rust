use std::io::Read;
use nalgebra::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let x: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut m: isize = 0;
    let mut y: isize = 0;
    let mut c: isize = std::isize::MIN;
    for i in 0..n {
        m += x[i].abs();
        y += x[i] * x[i];
        c = max(c, x[i].abs());
    }

    let y = (y as f64).sqrt();
    format!("{}\n{}\n{}", m, y, c)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
2 -1",
            "3
2.236067977499790
2"
        ),
        (
            r"10
3 -1 -4 1 -5 9 2 -6 5 -3",
            "39
14.387494569938159
9"
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