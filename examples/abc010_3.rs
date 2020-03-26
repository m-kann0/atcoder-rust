use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x_start: f64 = iterator.next().unwrap().parse().unwrap();
    let y_start: f64 = iterator.next().unwrap().parse().unwrap();
    let x_goal: f64 = iterator.next().unwrap().parse().unwrap();
    let y_goal: f64 = iterator.next().unwrap().parse().unwrap();
    let t: f64 = iterator.next().unwrap().parse().unwrap();
    let v: f64 = iterator.next().unwrap().parse().unwrap();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let x: f64 = iterator.next().unwrap().parse().unwrap();
        let y: f64 = iterator.next().unwrap().parse().unwrap();

        let required =
            ((x - x_start).powi(2) + (y - y_start).powi(2)).sqrt()
            + ((x_goal - x).powi(2) + (y_goal - y).powi(2)).sqrt();
        if required <= t * v {
            return String::from("YES");
        }
    }

    return String::from("NO");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 1 8 2 2 4
1
4 5",
            "NO"
        ),
        (
            r"1 1 8 2 2 6
1
4 5",
            "YES"
        ),
        (
            r"1 1 8 2 2 5
1
4 5",
            "YES"
        ),
        (
            r"7 7 1 1 3 4
3
8 1
1 7
9 9",
            "YES"
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