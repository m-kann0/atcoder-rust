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
    let mut a: Vec<isize> = vec![0; n + 2];
    for i in 1..n + 1 {
        a[i] = iterator.next().unwrap().parse().unwrap();
    }

    let mut total = 0;
    for i in 1..n + 2 {
        total += (a[i] - a[i - 1]).abs();
    }

    let mut ans = String::new();
    for i in 1..n + 1 {
        ans.push_str(
            &format!(
                "{}\n",
                total - (a[i] - a[i - 1]).abs() - (a[i + 1] - a[i]).abs() + (a[i + 1] - a[i - 1]).abs()
            )
        );
    }
    return ans.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3 5 -1",
            "12
8
10"
        ),
        (
            r"5
1 1 1 2 0",
            "4
4
4
2
4"
        ),
        (
            r"6
-679 -2409 -3258 3095 -3291 -4462",
            "21630
21630
19932
8924
21630
19288"
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