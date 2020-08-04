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

    let mut xs = Vec::new();
    let begin = if n > 1000 {
        n - 1000
    } else {
        0
    };
    for x in begin..=n {
        if x + f(x) == n {
            xs.push(x);
        }
    }

    let mut result = String::new();
    result.push_str(&format!("{}\n", xs.len()));
    for x in xs {
        result.push_str(&format!("{}\n", x));
    }
    result.trim().to_string()
}

fn f(mut n: usize) -> usize {
    let mut result = 0;
    while n > 0 {
        result += n % 10;
        n /= 10;
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8",
            "1
4"
        ),
        (
            r"101",
            "2
91
100"
        ),
        (
            r"108",
            "0"
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

#[test]
fn test2() {
    for x in 0..=10000 {
        let fx = f(x);
        println!("x: {}, f(x): {}, x + f(x): {}", x, fx, x + fx);
    }
}
