use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let _n: usize = iterator.next().unwrap().parse().unwrap();
    let d: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut l: Vec<usize> = Vec::new();
    let mut r: Vec<usize> = Vec::new();
    for _ in 0..d {
        l.push(iterator.next().unwrap().parse().unwrap());
        r.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut s: Vec<usize> = Vec::new();
    let mut t: Vec<usize> = Vec::new();
    for _ in 0..k {
        s.push(iterator.next().unwrap().parse().unwrap());
        t.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut current: Vec<usize> = s.clone();
    let mut result: Vec<usize> = vec![0; k];

    for i in 0..d {
        for j in 0..k {
            if result[j] != 0 {
                continue;
            }
            if current[j] < l[i] || current[j] > r[i] {
                continue;
            }
            if current[j] < t[j] {
                if t[j] <= r[i] {
                    current[j] = t[j];
                    result[j] = i + 1;
                } else {
                    current[j] = r[i];
                }
            } else {
                if t[j] >= l[i] {
                    current[j] = t[j];
                    result[j] = i + 1;
                } else {
                    current[j] = l[i];
                }
            }
        }
    }

    let mut out = String::new();
    for r in result {
        out.push_str(&format!("{}\n", r));
    }
    return out.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 10 3
1 5
3 6
7 10
5 8
4 4
1 4
2 9
1 3
1 1
4 5
1 6
2 7
10 1",
            "2
4
8"
        ),
        (
            r"10 10 4
1 2
2 4
3 6
4 8
5 10
9 10
7 8
5 6
3 5
1 3
10 1
3 8
2 4
1 3",
            "10
4
2
2"
        ),
        (
            r"314159265 10 1
1 10000
500 12031
1414 113232
111111 777777
666661 23423423
12345678 123456789
111111111 314159265
112334 235235235
1 223445
314 1592
1 314159265",
            "7"
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