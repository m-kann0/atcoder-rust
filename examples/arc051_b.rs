use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut fib: Vec<usize> = vec![0; 50];
    for i in 0..50 {
        if i <= 2 {
            fib[i] = i;
        } else {
            fib[i] = fib[i - 1] + fib [i - 2];
        }
    }

    let k: usize = iterator.next().unwrap().parse().unwrap();

    return format!("{} {}", fib[k + 1], fib[k]);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1",
            "1 1"
        ),
        (
            r"3",
            "4 5"
        ),
        (
            r"12",
            "314159265 358979323"
        ),
        (
            r"40",
            "1 2"
        )
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