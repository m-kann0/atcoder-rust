use std::io::Read;
use std::collections::HashMap;
use num_integer::gcd_lcm;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut zero: usize = 0;
    let mut mp: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    for _ in 0..n {
        let mut x: isize = iterator.next().unwrap().parse().unwrap();
        let mut y: isize = iterator.next().unwrap().parse().unwrap();
        if x == 0 && y == 0 {
            zero += 1;
            continue;
        }
        let g: isize = gcd_lcm(x, y).0;
        x /= g;
        y /= g;
        if y < 0 {
            x = -x;
            y = -y;
        }
        if y == 0 && x < 0 {
            x = -x;
            y = -y;
        }
        let rot90: bool = x <= 0;
        if rot90 {
            let tmp = y;
            y = -x;
            x = tmp;
        }
        if rot90 {
            (*mp.entry((x, y)).or_insert((0, 0))).0 += 1;
        } else {
            (*mp.entry((x, y)).or_insert((0, 0))).1 += 1;
        }
    }

    let mut ans: usize = 1;
    for x in mp {
        let s = (x.1).0;
        let t = (x.1).1;
        let mut now = 1;
        now += mod_pow(2, s as usize) - 1;
        now %= MOD;
        now += mod_pow(2, t as usize) - 1;
        now %= MOD;
        ans = ans * now % MOD;
    }
    if ans == 0 {
        ans = MOD - 1;
    } else {
        ans -= 1;
    }
    ans = ans + zero % MOD;
    return ans.to_string();
}

const MOD: usize = 1_000_000_007;

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