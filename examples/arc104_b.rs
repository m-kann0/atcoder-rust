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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let s: Vec<usize> = s.iter().map(|c| {
        match *c {
            'A' => 0,
            'T' => 1,
            'C' => 2,
            _ => 3,
        }
    }).collect();

    let mut count: Vec<Vec<usize>> = vec![vec![0; 4]; n + 1];
    for i in 0..n {
        for j in 0..4 {
            count[i + 1][j] = count[i][j];
        }
        count[i + 1][s[i]] += 1;
    }

    let mut ans: usize = 0;
    for l in 0..=n {
        for r in 0..=n {
            if l >= r {
                continue;
            }
            let mut t: Vec<usize> = vec![0; 4];
            for i in 0..4 {
                t[i] = count[r][i] - count[l][i];
            }
            if t[0] == t[1] && t[2] == t[3] {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 AGCT",
            "2"
        ),
        (
            r"4 ATAT",
            "4"
        ),
        (
            r"10 AAATACCGCG",
            "6"
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