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
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let a_original: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut a = a_original.clone();
    if a[0] > x {
        a[0] = x;
    }
    for i in 1..n {
        if a[i] + a[i - 1] > x {
            let dec = a[i] + a[i - 1] - x;
            if a[i] >= dec {
                a[i] -= dec;
            } else {
                a[i] = 0;
                a[i - 1] = x;
            }
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        ans += a_original[i] - a[i];
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
2 2 2",
            "1"
        ),
        (
            r"6 1
1 6 1 2 0 4",
            "11"
        ),
        (
            r"5 9
3 1 4 1 5",
            "0"
        ),
        (
            r"2 0
5 5",
            "10"
        ),
        (
            r"6 1
4 0 2 1 6 1",
            "11"
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