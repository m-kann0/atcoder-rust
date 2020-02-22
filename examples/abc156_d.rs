use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u32 = iterator.next().unwrap().parse().unwrap();
    let a: u32 = iterator.next().unwrap().parse().unwrap();
    let b: u32 = iterator.next().unwrap().parse().unwrap();
    println!("A");
    let (fac, finv) = init(n + 1);
    println!("B");
    let mut answer: u32 = 0;
    for i in 1..(n + 1) {
        if i == a || i == b {
            continue
        }
        answer = (answer + comb(n, i, &fac, &finv)) % MOD;
    }
    println!("C");
    return format!("{}", answer);
}

const MOD: u32 = 1000000007;

fn init(n: u32) -> (Vec<u32>, Vec<u32>) {
    let mut fac = vec![0; n as usize];
    println!("A1");
    let mut finv = vec![0; n as usize];
    println!("A2");
    let mut inv = vec![0; n as usize];
    println!("A3");

    fac[0] = 1;
    fac[1] = 1;
    finv[0] = 1;
    finv[1] = 1;
    inv[1] = 1;
    for i in 2..n as usize {
        fac[i] = (fac[i - 1] as u64 * i as u64 % MOD as u64) as u32;
        inv[i] = MOD - (inv[MOD as usize % i] as u64 * (MOD as u64 / i as u64) % MOD as u64) as u32;
        finv[i] = (finv[i - 1] as u64 * inv[i] as u64 % MOD as u64) as u32;
    }

    (fac, finv)
}

fn comb(n: u32, r: u32, fac: &Vec<u32>, finv: &Vec<u32>) -> u32 {
    return fac[n as usize] * (finv[r as usize] * finv[n as usize - r as usize] % MOD) % MOD;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 1 3",
            "7"
        ),
        (
            r"1000000000 141421 173205",
            "34076506"
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
