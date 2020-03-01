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
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut lumps: Vec<Vec<usize>> = Vec::with_capacity(m);
    for _i in 0..m {
        let mut vec: Vec<usize> = Vec::new();
        let k: usize = iterator.next().unwrap().parse().unwrap();
        for _j in 0..k {
            vec.push(iterator.next().unwrap().parse::<usize>().unwrap() - 1);
        }
        lumps.push(vec);
    }

    let mut p: Vec<usize> = Vec::with_capacity(m);
    for _i in 0..m {
        p.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut ans = 0;
    for i in 0..(2_usize.pow(n as u32)) {
        let pattern = make_pattern(i, n);
        let mut is_ok = true;
        for (j, lump) in lumps.iter().enumerate() {
            let mut count = 0;
            for &l in lump {
                if pattern[l] {
                    count += 1;
                }
            }
            if count % 2 != p[j] {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            ans += 1;
        }
    }

    return ans.to_string();
}

fn make_pattern(i: usize, len: usize) -> Vec<bool> {
    let mut n = i;
    let mut result = Vec::with_capacity(len);
    while n > 0 {
        if n % 2 == 1 {
            result.push(true);
        } else {
            result.push(false);
        }
        n /= 2;
    }
    while result.len() < len {
        result.push(false);
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2
2 1 2
1 2
0 1",
            "1"
        ),
        (
            r"2 3
2 1 2
1 1
1 2
0 0 1",
            "0"
        ),
        (
            r"5 2
3 1 2 5
2 2 3
1 0",
            "8"
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