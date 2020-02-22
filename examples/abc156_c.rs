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

    let mut x: Vec<usize> = Vec::with_capacity(n);
    let mut total: usize = 0;
    for _i in 0..n {
        let xi: usize = iterator.next().unwrap().parse().unwrap();
        x.push(xi);
        total += xi;
    }

    let average = ((total as f64) / (n as f64)).round() as usize;

    let mut answer: usize = 0;
    for i in 0..n {
        answer += (x[i] as i64 - average as i64).pow(2) as usize;
    }

    return format!("{}", answer);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
1 4",
            "5"
        ),
        (
            r"7
14 14 2 13 56 2 37",
            "2354"
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
