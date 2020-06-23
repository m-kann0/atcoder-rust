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
    let d: usize = iterator.next().unwrap().parse().unwrap();
    let mut x: Vec<Vec<isize>> = Vec::new();
    for _ in 0..n {
        let mut v: Vec<isize> = Vec::new();
        for _ in 0..d {
            v.push(iterator.next().unwrap().parse().unwrap());
        }
        x.push(v);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in 0..n {
            if i >= j {
                continue;
            }
            let mut s = 0;
            for k in 0..d {
                s += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
            }

            let mut l = 0;
            while l * l <= s {
                if l * l == s {
                    ans += 1;
                    break;
                }
                l += 1;
            }
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
1 2
5 5
-2 8",
            "1"
        ),
        (
            r"3 4
-3 7 8 2
-12 1 10 2
-2 8 9 3",
            "2"
        ),
        (
            r"5 1
1
2
3
4
5",
            "10"
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