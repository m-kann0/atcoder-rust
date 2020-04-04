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
    let c: usize = iterator.next().unwrap().parse().unwrap();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut i = 0;
    let mut l: Vec<usize> = Vec::new();
    while l.len() < k {
        if s[i] == 'o' {
            l.push(i);
            i += c + 1;
        } else {
            i += 1;
        }
    }

    let mut i: isize = n as isize - 1;
    let mut r: Vec<usize> = Vec::new();
    while r.len() < k {
        if s[i as usize] == 'o' {
            r.push(i as usize);
            i -= c as isize + 1;
        } else {
            i -= 1;
        }
    }

    let mut ans = String::new();
    for (&l, &r) in l.iter().zip(r.iter().rev()) {
        if l == r {
            ans.push_str(&format!("{}\n", l + 1));
        }
    }
    return ans.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"11 3 2
ooxxxoxxxoo",
            "6"
        ),
        (
            r"5 2 3
ooxoo",
            "1
5"
        ),
        (
            r"5 1 0
ooooo",
            ""
        ),
        (
            r"16 4 3
ooxxoxoxxxoxoxxo",
            "11
16"
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
