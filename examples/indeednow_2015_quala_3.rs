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
    let mut counts: Vec<usize> = vec![0; 1_000_001];
    for _ in 0..n {
        let si: usize = iterator.next().unwrap().parse().unwrap();
        if si == 0 {
            continue;
        }
        counts[si] += 1;
    }

    let mut sum: Vec<usize> = vec![0; 1_000_002];
    sum[1_000_000] = counts[1_000_000];
    for i in (0..1_000_000).rev() {
        sum[i] = sum[i + 1] + counts[i];
    }

    let mut result = String::new();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..q {
        let k: usize = iterator.next().unwrap().parse().unwrap();
        let mut ok: isize = 1_000_001;
        let mut ng: isize = -1;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if sum[mid as usize] <= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        result.push_str(&format!("{}\n", ok));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15
0
0
0
1
1
2
3
4
5
6
6
6
8
9
10
3
0
4
12",
            "11
7
0"
        ),
        (
            r"9
3
3
3
2
2
2
1
1
1
1
4",
            "3"
        ),
        (
            r"4
0
0
0
0
1
0",
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