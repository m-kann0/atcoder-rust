use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

const N_MAX: i64 = 1000000000;

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: i64 = iterator.next().unwrap().parse().unwrap();
    let b: i64 = iterator.next().unwrap().parse().unwrap();
    let x: i64 = iterator.next().unwrap().parse().unwrap();

//    if can_buy(N_MAX, a, b, x) {
//        return N_MAX.to_string();
//    }
//
//    if !can_buy(1, a, b, x) {
//        return "0".to_string();
//    }

    let mut upper: i64 = N_MAX + 1;
    let mut lower: i64 = 0;
    loop {
        let n = (upper + lower) / 2;
        if can_buy(n, a, b, x) {
            lower = n;
        } else {
            upper = n;
        }

        if lower + 1 >= upper {
            break;
        }
    }
    return lower.to_string();
}

fn can_buy(n: i64, a: i64, b: i64, x: i64) -> bool {
    return a * n + b * d(n) <= x;
}

fn d(n: i64) -> i64 {
    return n.to_string().len() as i64;
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"10 7 100"
        ),
        "9"
    );
    assert_eq!(
        solve(
            r"2 1 100000000000"
        ),
        "1000000000"
    );
    assert_eq!(
        solve(
            r"1000000000 1000000000 100"
        ),
        "0"
    );
    assert_eq!(
        solve(
            r"1234 56789 314159265"
        ),
        "254309"
    );
}
