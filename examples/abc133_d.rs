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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let s: usize = a.iter().sum();

    let mut b: Vec<usize> = vec![0; n];
    b[0] = s;
    for i in 0..n {
        if i % 2 == 1 {
            b[0] -= 2 * a[i];
        }
    }

    for i in 1..n {
        b[i] = 2 * a[i - 1] - b[i - 1];
    }
    return b.iter().map(|it| it.to_string()).collect::<Vec<String>>().join(" ");
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
2 2 4",
            "4 0 4"
        ),
        (
            r"5
3 8 7 5 5",
            "2 4 12 2 8"
        ),
        (
            r"3
1000000000 1000000000 0",
            "0 2000000000 0"
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