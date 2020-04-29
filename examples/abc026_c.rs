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
    let mut b: Vec<usize> = vec![0];
    for i in 2.. {
        if i > n {
            break;
        }
        b.push(iterator.next().unwrap().parse().unwrap());
    }

    let ans = salary(1, &b);

    return ans.to_string();
}

fn salary(x: usize, b: &Vec<usize>) -> usize {
    let mut salaries: Vec<usize> = Vec::new();
    for i in 0..b.len() {
        if b[i] == x {
            salaries.push(salary(i + 1, b));
        }
    }
    if salaries.is_empty() {
        return 1;
    }
    let min = salaries.iter().min().unwrap();
    let max = salaries.iter().max().unwrap();
    return max + min + 1;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
1
1
1
1",
            "3"
        ),
        (
            r"7
1
1
2
2
3
3",
            "7"
        ),
        (
            r"6
1
2
3
3
2",
            "11"
        ),
        (
            r"10
1
2
3
4
5
6
7
8
9",
            "1023"
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