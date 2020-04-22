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
    let mut count_l: i64 = 0;
    let mut count_r: i64 = 0;
    let mut count_u: i64 = 0;
    let mut count_d: i64 = 0;
    let mut count_unknown: i64 = 0;
    for &c in &s {
        if c == 'L' {
            count_l += 1;
        } else if c == 'R' {
            count_r += 1;
        } else if c == 'U' {
            count_u += 1;
        } else if c == 'D' {
            count_d += 1;
        } else {
            count_unknown += 1;
        }
    }

    let current_x = count_l - count_r;
    let current_y = count_u - count_d;

    let t: u8 = iterator.next().unwrap().parse().unwrap();
    if t == 1 {
        let ans = current_x.abs() + current_y.abs() + count_unknown;
        return ans.to_string();
    }

    let d = current_x.abs() + current_y.abs();
    return if d >= count_unknown {
        (d - count_unknown).to_string()
    } else {
        if (count_unknown - d) % 2 == 0 {
            "0".to_string()
        } else {
            "1".to_string()
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"UL?
1",
            "3"
        ),
        (
            r"UD?
1",
            "1"
        ),
        (
            r"UUUU?DDR?LLLL
1",
            "7"
        ),
        (
            r"UULL?
2",
            "3"
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