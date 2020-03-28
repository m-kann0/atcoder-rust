use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();
    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();

    let mut p: Vec<usize> = (0..a)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();
    let mut q: Vec<usize> = (0..b)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();
    let r: Vec<usize> = (0..c)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    p.sort();
    p.reverse();
    q.sort();
    q.reverse();

    let mut d: Vec<usize> = Vec::new();
    for i in 0..x {
        d.push(p[i]);
    }
    for i in 0..y {
        d.push(q[i]);
    }
    for i in 0..c {
        d.push(r[i]);
    }

    d.sort();

    let mut ans: usize = 0;
    for _ in 0..x + y {
        ans += d.pop().unwrap();
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 2 2 2 1
2 4
5 1
3",
            "12"
        ),
        (
            r"2 2 2 2 2
8 6
9 1
2 1",
            "25"
        ),
        (
            r"2 2 4 4 4
11 12 13 14
21 22 23 24
1 2 3 4",
            "74"
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