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
    let colors: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    if colors.iter().all(|it| *it == 0) || colors.iter().all(|it| *it == 1) {
        return "-1".to_string();
    }

    let mut counts: Vec<usize> = Vec::new();
    let mut prev = colors[0];
    let mut count = 1;
    for i in 1..n {
        let current = colors[i];
        if prev == current {
            count += 1;
        } else {
            counts.push(count);
            count = 1;
        }
        prev = current;
    }
    counts.push(count);

    if colors[0] == colors[n - 1] {
        counts[0] += counts.pop().unwrap();
    }

    let m = counts.iter().max().unwrap();
    let ans = (*m + 1) / 2;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
0
1
1
1
0",
            "2"
        ),
        (
            r"6
1
1
0
1
1
1",
            "3"
        ),
        (
            r"3
1
1
1",
            "-1"
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