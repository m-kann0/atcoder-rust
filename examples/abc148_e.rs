use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n_str = iterator.next().unwrap();
    let n: u64 = n_str.parse().unwrap();

    if n % 2 == 1 {
        return String::from("0");
    }

    let keta = n_str.len();

    let mut ans = 0;
    for i in 1..keta {
        let d: u64 = 10_u64.pow(i as u32);
        ans += n / d;
    }

    return ans.to_string();
}

// fn main() {
//     let mut n = 10;
//     for _i in 0..10 {
//         println!("{}: {}", n, count(n));
//         n += 10;
//     }
// }
//
// fn count(n: u64) -> u64 {
//     let mut result = f(n);
//     let mut c = 0;
//     while result % 10 == 0 {
//         c += 1;
//         result /= 10;
//     }
//     c
// }
//
// fn f(n: u64) -> u64 {
//     if n < 2 {
//         1
//     } else {
//         n * f(n - 2)
//     }
// }

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"12",
            "1"
        ),
        (
            r"5",
            "0"
        ),
        (r"100", r"11"),
        (r"200", r"22"),
        (r"300", r"33"),
        (r"400", r"44"),
        (r"500", r"55"),
        (r"600", r"66"),
        (r"700", r"77"),
        (r"800", r"88"),
        (r"900", r"99"),
        (r"1000", r"111"),
        (r"1010", r"112"),
        (r"1090", r"120"),
        (r"1100", r"122"),
        (r"10000", r"1111"),
        (
            r"1000000000000000000",
            "124999999999999995"
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