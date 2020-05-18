use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut counts: HashMap<(isize, usize, usize), usize> = HashMap::new();
    let mut zero: usize = 0;
    for _ in 0..n {
        let mut ai: isize = iterator.next().unwrap().parse().unwrap();
        let mut bi: isize = iterator.next().unwrap().parse().unwrap();
        if ai == 0 && bi == 0 {
            zero += 1;
            continue;
        }
        let mut sign: isize = 1;
        if ai < 0 {
            sign *= -1;
            ai *= -1;
        }
        if bi < 0 {
            sign *= -1;
            bi *= -1;
        }
        if ai == 0 || bi == 0 {
            sign = 1;
        }
        let ai: usize = ai as usize;
        let bi: usize = bi as usize;
        let g: usize = gcd(ai, bi);
        let ai = ai / g;
        let bi = bi / g;
        *counts.entry((sign, ai, bi)).or_insert(0) += 1;
    }

    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let keys: Vec<(isize, usize, usize)> = counts.keys().into_iter().map(|it| *it).collect();
    for key in keys {
        if let Some(count1) = counts.get(&key) {
            let key2 = if key.1 != 0 && key.2 != 0 {
                (-key.0, key.2, key.1)
            } else {
                (key.0, key.2, key.1)
            };
            if let Some(count2) = counts.get(&key2) {
                pairs.push((*count1, *count2));
                counts.remove(&key2);
            } else {
                pairs.push((*count1, 0));
            }
            counts.remove(&key);
        }
    }

    let mut ans: usize = 1;
    for pair in pairs {
        let mut now: usize = 1;
        let s = pair.0;
        let t = pair.1;
        now += mod_pow(2, s) - 1;
        now %= MOD;
        now += mod_pow(2, t) - 1;
        now %= MOD;

        ans *= now;
        ans %= MOD;
    }
    ans += zero;
    ans %= MOD;
    if ans > 0 {
        ans -= 1;
    } else {
        ans = MOD - 1;
    }

    return ans.to_string();
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    return x;
}

const MOD: usize = 1000000007;

fn mod_pow(a: usize, b: usize) -> usize {
    let mut result = 1;
    for _ in 0..b {
        result = result * a % MOD;
    }
    return result
}


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 2
-1 1
2 -1",
            "5"
        ),
        (
            r"10
3 2
3 2
-1 1
2 -1
-3 -9
-8 12
7 7
8 1
8 2
8 4",
            "479"
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