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
    let s: String = iterator.next().unwrap().to_string();

    let x: Vec<isize> = s.chars().map(|c| c.to_digit(10).unwrap() as isize).collect();
    let pc: isize = x.iter().sum();

    let mut ans: Vec<isize> = vec![0; n];
    for b in 0..2 {
        let npc = if b == 0 {
            pc + 1
        } else {
            pc - 1
        };

        if npc <= 0 {
            continue;
        }

        let mut r0: isize = 0;
        for i in 0..n {
            r0 = (r0 * 2) % npc;
            r0 += x[i];
        }

        let mut k: isize = 1;
        for i in (0..n).rev() {
            if x[i] == b {
                let mut r = r0;
                if b == 0 {
                    r = (r + k) % npc;
                } else {
                    r = (r - k + npc) % npc;
                }
                ans[i] = f(r) + 1;
            }
            k = (k * 2) % npc;
        }
    }

    let mut result = String::new();
    for i in 0..n {
        result.push_str(&format!("{}\n", ans[i]));
    }
    result.trim().to_string()
}

fn f(x: isize) -> isize {
    if x == 0 {
        return 0;
    }
    return f(x % pcnt(x)) + 1;
}

fn pcnt(mut x: isize) -> isize {
    let mut cnt = 0;
    while x > 0 {
        if x % 2 == 1 {
            cnt += 1;
        }
        x /= 2;
    }
    return cnt;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
011",
            "2
1
1"
        ),
        (
            r"23
00110111001011011001110",
            "2
1
2
2
1
2
2
2
2
2
2
2
2
2
2
2
2
2
2
2
2
1
3"
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

#[test]
fn test2() {
    let mut a: usize = 1;
    for i in 0..200000 {
        println!("2^{} = {}", i, a);
        a *= 2;
    }
}