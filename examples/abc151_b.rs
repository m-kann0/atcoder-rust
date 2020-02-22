use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: i32 = iterator.next().unwrap().parse().unwrap();
    let k: i32 = iterator.next().unwrap().parse().unwrap();
    let m: i32 = iterator.next().unwrap().parse().unwrap();

    let m_total: i32 = n * m;
    let mut total: i32 = 0;
    for _ in 0..(n - 1) {
        total += iterator.next().unwrap().parse::<i32>().unwrap();
    }

    let ans: i32 = m_total - total;

    if ans > k {
        return "-1".to_string();
    }
    if ans < 0 {
        return "0".to_string();
    }

    return ans.to_string();
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"5 10 7
8 10 3 6"
        ),
        "8"
    );
    assert_eq!(
        solve(
            r"4 100 60
100 100 100"
        ),
        "0"
    );
    assert_eq!(
        solve(
            r"4 100 60
0 0 0"
        ),
        "-1"
    );
}
