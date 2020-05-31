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
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut mk: Vec<usize> = vec![0; k];
    for i in 1..(n + 1) {
        mk[i % k] += 1;
    }

    let mut ans: usize = 0;
    for a in 0..k {
        let b = (k - a) % k;
        let c = (k - a) % k;
        if (b + c) % k != 0 {
            continue;
        }
        ans += mk[a] * mk[b] * mk[c];
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2",
            "9"
        ),
        (
            r"5 3",
            "1"
        ),
        (
            r"31415 9265",
            "27"
        ),
        (
            r"35897 932",
            "114191"
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