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
    let divisors = enumerate_divisors(n);

    let mut result = String::new();
    for divisor in divisors {
        result.push_str(&format!("{}\n", divisor));
    }
    result.trim().to_string()
}

fn enumerate_divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            divisors.push(i);
            if n / i != i {
                divisors.push(n / i);
            }
        }
        i += 1;
    }
    divisors.sort();
    return divisors;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6",
            "1
2
3
6"
        ),
        (
            r"720",
            "1
2
3
4
5
6
8
9
10
12
15
16
18
20
24
30
36
40
45
48
60
72
80
90
120
144
180
240
360
720"
        ),
        (
            r"1000000007",
            "1
1000000007"
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