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
    let mut a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const MOD: isize = 1_000_000_007;

    if k == n {
        let mut ans: isize = 1;
        for i in 0..n {
            ans = ans * a[i] % MOD;
        }
        return if ans >= 0 {
            ans.to_string()
        } else {
            (ans + MOD).to_string()
        }
    }

    if k % 2 == 1 && a.iter().all(|it| *it < 0) {
        a.sort();
        a.reverse();

        let mut ans: isize = 1;
        for i in 0..k {
            ans = ans * a[i] % MOD;
        }
        return if ans >= 0 {
            ans.to_string()
        } else {
            (ans + MOD).to_string()
        }
    }

    a.sort_by_key(|it| it.abs());

    let mut is_minus = false;
    let mut is_zero = false;
    let mut b: Vec<isize> = Vec::new();
    for _ in 0..k {
        let ai = a.pop().unwrap();
        if ai < 0 {
            is_minus = !is_minus;
        } else if ai == 0 {
            is_zero = true;
        }
        b.push(ai);
    }

    if !is_minus || is_zero {
        let mut ans = 1;
        for bi in b {
            ans = ans * bi % MOD;
        }
        return if ans >= 0 {
            ans.to_string()
        } else {
            (ans + MOD).to_string()
        }
    }

    let mut lp = None;
    for bi in b.iter().rev() {
        if *bi >= 0 {
            lp = Some(*bi);
            break;
        }
    }

    let mut lm = None;
    for bi in b.iter().rev() {
        if *bi < 0 {
            lm = Some(*bi);
            break;
        }
    }

    let mut np = None;
    for ai in a.iter().rev() {
        if *ai >= 0 {
            np = Some(*ai);
            break;
        }
    }

    let mut nm = None;
    for ai in a.iter().rev() {
        if *ai < 0 {
            nm = Some(*ai);
            break;
        }
    }

    if lp != None && nm != None && lm != None && np != None {
        if lp.unwrap() * np.unwrap() >= lm.unwrap() * nm.unwrap() {
            b.remove(b.iter().position(|x| *x == lm.unwrap()).unwrap());
            b.push(np.unwrap());
        } else {
            b.remove(b.iter().position(|x| *x == lp.unwrap()).unwrap());
            b.push(nm.unwrap());
        }
    } else if lp != None && nm != None {
        b.remove(b.iter().position(|x| *x == lp.unwrap()).unwrap());
        b.push(nm.unwrap());
    } else if lm != None && np != None {
        b.remove(b.iter().position(|x| *x == lm.unwrap()).unwrap());
        b.push(np.unwrap());
    }

    let mut ans = 1;
    for bi in b {
        ans = ans * bi % MOD;
    }
    return if ans >= 0 {
        ans.to_string()
    } else {
        (ans + MOD).to_string()
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2
1 2 -3 -4",
            "12"
        ),
        (
            r"4 3
-1 -2 -3 -4",
            "1000000001"
        ),
        (
            r"2 1
-1 1000000000",
            "1000000000"
        ),
        (
            r"10 10
1000000000 100000000 10000000 1000000 100000 10000 1000 100 10 1",
            "999983200"
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