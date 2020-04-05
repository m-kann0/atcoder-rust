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
    let x: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut sorted: Vec<usize> = x.clone();
    sorted.sort();

    let median: usize = sorted[n / 2];
    let pre_median = sorted[n / 2 - 1];

    let mut result = String::new();
    for xi in x {
        if xi >= median {
            result.push_str(&format!("{}\n", pre_median));
        } else {
            result.push_str(&format!("{}\n", median));
        }
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
2 4 4 3",
            "4
3
3
4"
        ),
        (
            r"2
1 2",
            "2
1"
        ),
        (
            r"6
5 5 4 4 3 3",
            "4
4
4
4
4
4"
        ),
        (
            r"2
2 2",
            "2
2"
        ),
        (
            r"8
1 2 3 4 5 6 7 8",
            "5
5
5
5
4
4
4
4"
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