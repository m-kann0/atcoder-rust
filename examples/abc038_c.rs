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
    let a: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let mut ranges: Vec<usize> = Vec::new();

    let mut range: usize = 1;
    let mut prev: usize = a[0];
    for i in 1..n {
        if a[i] > prev {
            range += 1;
        } else {
            if range >= 2 {
                ranges.push(range);
            }
            range = 1;
        }
        prev = a[i];
    }
    if range >= 2 {
        ranges.push(range);
    }

    let mut ans: usize = n;
    for range in ranges {
        ans += range * (range - 1) / 2;
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1 2 3 2 1",
            "8"
        ),
        (
            r"4
1 2 3 4",
            "10"
        ),
        (
            r"6
3 3 4 1 2 2",
            "8"
        ),
        (
            r"6
1 5 2 3 4 2",
            "10"
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