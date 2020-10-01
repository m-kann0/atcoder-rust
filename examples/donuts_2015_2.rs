use std::io::Read;
use std::collections::HashSet;
use std::cmp::max;

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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut bonus: Vec<usize> = Vec::new();
    let mut conditions: Vec<HashSet<usize>> = Vec::new();
    for _ in 0..m {
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        let mut condition: HashSet<usize> = HashSet::new();
        for _ in 0..ci {
            let i: usize = iterator.next().unwrap().parse().unwrap();
            let i = i - 1;
            condition.insert(i);
        }
        bonus.push(bi);
        conditions.push(condition);
    }

    let mut ans: usize = 0;
    for i in 0..2_u32.pow(n as u32) {
        let bin: Vec<char> = binary_string(i as usize, n).chars().collect();
        if bin.iter().filter(|it| **it == '1').count() != 9 {
            continue;
        }
        let mut point = 0;
        let mut member: HashSet<usize> = HashSet::new();
        for j in 0..n {
            if bin[j] == '1' {
                point += a[j];
                member.insert(j);
            }
        }
        for j in 0..m {
            if conditions[j].intersection(&member).count() >= 3 {
                point += bonus[j];
            }
        }
        ans = max(ans, point);
    }
    ans.to_string()
}

fn binary_string(n: usize, keta: usize) -> String {
    return format!("{:0>1$b}", n, keta);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 1
100 200 300 400 500 600 700 800 900 1000
1000 3 1 2 3",
            "6100"
        ),
        (
            r"12 10
1000 1000 1000 1000 1000 1000 1000 1000 1000 1000 1000 1000
1000 4 1 2 4 7
1000 4 1 9 11 12
1000 4 3 5 8 9
1000 4 6 10 11 12
1000 4 2 4 7 10
1000 4 1 8 9 10
1000 3 1 9 12
1000 4 3 8 11 12
1000 4 1 2 3 4
1000 4 7 8 9 10",
            "19000"
        ),
        (
            r"13 8
328 781 104 102 132 108 100 102 104 108 168 102 100
184 4 10 11 3 4
190 4 9 6 2 5
282 6 9 1 3 12 10 8
205 8 13 10 1 12 7 2 8 11
122 8 13 5 4 3 8 9 12 10
112 7 11 6 12 8 2 13 5
102 4 4 13 6 12
109 6 7 2 13 1 8 6",
            "3239"
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