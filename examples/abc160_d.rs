use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();
    let x: isize = iterator.next().unwrap().parse().unwrap();
    let y: isize = iterator.next().unwrap().parse().unwrap();

    let mut counts: Vec<usize> = vec![0; (n + 1) as usize];

    for i in 1..n {
        for j in i + 1 .. n + 1 {
            let mut count = j - i;
            count = min(count, (x - i).abs() + 1 + (j - y).abs());
            counts[count as usize] += 1;
        }
    }

    let mut ans = String::new();
    for i in 1..n {
        ans.push_str(&format!("{}\n", counts[i as usize]))
    }
    return ans.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2 4",
            "5
4
1
0"
        ),
        (
            r"3 1 3",
            "3
0"
        ),
        (
            r"7 3 7",
            "7
8
4
2
0
0"
        ),
        (
            r"10 4 8",
            "10
12
10
8
4
1
0
0
0"
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