use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut min_capacity: usize = iterator.next().unwrap().parse().unwrap();
    min_capacity = min(min_capacity, iterator.next().unwrap().parse().unwrap());
    min_capacity = min(min_capacity, iterator.next().unwrap().parse().unwrap());
    min_capacity = min(min_capacity, iterator.next().unwrap().parse().unwrap());
    min_capacity = min(min_capacity, iterator.next().unwrap().parse().unwrap());

    if min_capacity >= n {
        return "5".to_string()
    }

    if n % min_capacity == 0 {
        return format!("{}", 5 + (n / min_capacity - 1));
    }

    return format!("{}", 5 + (n / min_capacity));
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
3
2
4
3
5",
            "7"
        ),
        (
            r"6
3
2
4
3
5",
            "7"
        ),
        (
            r"7
3
2
4
3
5",
            "8"
        ),
        (
            r"10
123
123
123
123
123",
            "5"
        ),
        (
            r"10000000007
2
3
5
7
11",
            "5000000008"
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