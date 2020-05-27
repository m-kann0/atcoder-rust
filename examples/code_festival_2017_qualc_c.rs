use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut t: Vec<char> = Vec::new();
    let mut indexes: Vec<usize> = Vec::new();
    for i in 0..s.len() {
        if s[i] != 'x' {
            t.push(s[i]);
            indexes.push(i);
        }
    }

    if t.len() == 0 {
        return "0".to_string()
    }

    if !is_kaibun(&t) {
        return "-1".to_string();
    }

    let (l, r) = if indexes.len() % 2 == 0 {
        (indexes[indexes.len() / 2 - 1], indexes[indexes.len() / 2])
    } else {
        (indexes[indexes.len() / 2], indexes[indexes.len() / 2])
    };
    // println!("l = {}, r = {}", l, r);

    // 中心から左方向に見ていく
    let mut counts_l: Vec<isize> = Vec::new();
    let mut c = 0;
    for i in (0..l).rev() {
        // println!("{}", s[i]);
        if s[i] == 'x' {
            c += 1;
        } else {
            counts_l.push(c);
            c = 0;
        }
    }
    counts_l.push(c);
    // println!("{:?}", counts_l);

    // 中心から右方向に見ていく
    let mut counts_r: Vec<isize> = Vec::new();
    let mut c = 0;
    for i in (r + 1)..s.len() {
        // println!("{}", s[i]);
        if s[i] == 'x' {
            c += 1;
        } else {
            counts_r.push(c);
            c = 0;
        }
    }
    counts_r.push(c);

    // println!("{:?}", counts_r);

    let mut ans = 0;
    for (l, r) in counts_l.iter().zip(counts_r.iter()) {
        ans += (*l - *r).abs();
    }
    return ans.to_string();
}

fn is_kaibun(t: &Vec<char>) -> bool {
    for i in 0..(t.len() / 2) {
        if t[i] != t[t.len() - 1 - i] {
            return false;
        }
    }
    return true;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"xabxa",
            "2"
        ),
        (
            r"ab",
            "-1"
        ),
        (
            r"a",
            "0"
        ),
        (
            r"oxxx",
            "3"
        ),
        (
            r"xxx",
            "0"
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