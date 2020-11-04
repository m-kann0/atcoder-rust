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
    let w: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut current = Vec::new();
    for i in 0..n {
        // current.sort();
        let mut can_put = false;
        for j in 0..current.len() {
            if current[j] >= w[i] {
                current[j] = w[i];
                can_put = true;
                break;
            }
        }
        if !can_put {
            current.push(w[i]);
        }
    }
    current.len().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
4
3
1
2
1",
            "2"
        ),
        (
            r"7
93
249
150
958
442
391
25",
            "3"
        ),
        (
            r"4
100
100
100
100",
            "1"
        ),
        (
            r"6
5
10
15
20
25
30",
            "6"
        ),
        (
            r"15
3
1
4
1
5
9
2
6
5
3
5
8
9
7
9",
            "6"
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