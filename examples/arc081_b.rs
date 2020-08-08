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
    let s1: Vec<char> = iterator.next().unwrap().chars().collect();
    let s2: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut is_tate: Vec<bool> = Vec::new();
    let mut i = 0;
    while i < n {
        if s1[i] == s2[i] {
            is_tate.push(true);
        } else {
            is_tate.push(false);
            i += 1;
        }
        i += 1;
    }

    let mut ans: usize = if is_tate[0] {
        3
    } else {
        3 * 2
    };

    const MOD: usize = 1_000_000_007;
    for i in 1..is_tate.len() {
        if is_tate[i - 1] {
            ans *= 2;
        } else {
            if !is_tate[i] {
                ans *= 3;
            }
        }
        ans %= MOD;
    }

    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
aab
ccb",
            "6"
        ),
        (
            r"1
Z
Z",
            "3"
        ),
        (
            r"52
RvvttdWIyyPPQFFZZssffEEkkaSSDKqcibbeYrhAljCCGGJppHHn
RLLwwdWIxxNNQUUXXVVMMooBBaggDKqcimmeYrhAljOOTTJuuzzn",
            "958681902"
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