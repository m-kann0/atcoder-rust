use std::io::Read;
use std::collections::HashSet;

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
    let mut phrases = HashSet::new();
    for _ in 0..n {
        let t = iterator.next().unwrap().to_string();
        phrases.insert(t);
    }

    const MOD: usize = 1_000_000_007;

    let mut dp: Vec<usize> = vec![0; s.len() + 1];
    dp[0] = 1;
    for i in 0..s.len() {
        let mut phrase = String::new();
        for j in i..s.len() {
            phrase.push(s[j]);
            if phrases.contains(&phrase) {
                dp[j + 1] += dp[i];
                dp[j + 1] %= MOD;
            }
        }
    }
    dp[s.len()].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
eternalstaticfinal
eternal
static
final",
            "1"
        ),
        (
            r"5
eternalstaticfinal
eternal
static
final
fin
al",
            "2"
        ),
        (
            r"5
abcdef
abc
def
abcdef
d
ef",
            "3"
        ),
        (
            r"5
aaaa
a
aa
aaa
aaaa
b",
            "8"
        ),
        (
            r"10
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
a
aa
aaa
aaaa
aaaaa
aaaaaa
aaaaaaa
aaaaaaaa
aaaaaaaaa
aaaaaaaaaa",
            "146491918"
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