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
    let mut c: Vec<usize> = Vec::new();
    let mut s: Vec<usize> = Vec::new();
    let mut f: Vec<usize> = Vec::new();
    for _ in 0..(n - 1) {
        c.push(iterator.next().unwrap().parse().unwrap());
        s.push(iterator.next().unwrap().parse().unwrap());
        f.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut result = String::new();
    for i in 0..n {
        let mut current: usize = i;
        let mut t: usize = 0;
        while current < n - 1 {
            if t < s[current] {
                t = s[current];
            } else if t % f[current] != 0 {
                t += f[current] - t % f[current];
            }
            t += c[current];
            current += 1;
        }
        result.push_str(&format!("{}\n", t));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
6 5 1
1 10 1",
            "12
11
0"
        ),
        (
            r"4
12 24 6
52 16 4
99 2 2",
            "187
167
101
0"
        ),
        (
            r"4
12 13 1
44 17 17
66 4096 64",
            "4162
4162
4162
0"
        ),
        (
            r"3
10 5 1
10 10 10",
            "30
20
0"
        ),
        (
            r"3
10 6 1
10 10 10",
            "30
20
0"
        ),
        (
            r"3
10 4 1
10 10 10",
            "30
20
0"
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