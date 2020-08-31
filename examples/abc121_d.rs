use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    let k = keta(b);

    if a == 0 {
        a = 1;
    }

    let xor_b = make(b, k);
    let xor_a = make(a - 1, k);

    let xor: Vec<bool> = xor_b.iter().zip(xor_a.iter())
        .map(|(b, a)| b != a)
        .collect();

    let mut ans: usize = 0;
    let mut x: usize = 1;
    for i in (0..k).rev() {
        if xor[i] {
            ans += x;
        }
        x *= 2;
    }
    ans.to_string()
}

fn keta(mut n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut k = 0;
    while n > 0 {
        n /= 2;
        k += 1;
    }
    k
}

fn make(n: usize, k: usize) -> Vec<bool> {
    let mut result = vec![false; k];
    let mut x = 2;
    for i in (0..k).rev() {
        let mut count = (n + 1) / x * x / 2;
        if (n + 1) % x > x / 2 {
            count += (n + 1) % x - x / 2;
        }
        result[i] = count % 2 == 1;
        x *= 2;
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 4",
            "5"
        ),
        (
            r"123 456",
            "435"
        ),
        (
            r"123456789012 123456789012",
            "123456789012"
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