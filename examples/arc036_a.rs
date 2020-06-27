use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let t: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    for i in 2..n {
        if t[i - 2] + t[i - 1] + t[i] < k {
            return (i + 1).to_string();
        }
    }

    return "-1".to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 1080
300
420
420
180
360",
            "4"
        ),
        (
            r"5 180
60
60
60
60
60",
            "-1"
        ),
        (
            r"5 4230
360
360
360
360
360",
            "3"
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