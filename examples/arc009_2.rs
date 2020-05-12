use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut b: Vec<usize> = vec![0; 10];
    for i in 0..10 {
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        b[bi] = i
    }

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let sort_key = generate_sort_key(&b, ai);
        a.push((sort_key, ai));
    }

    a.sort();

    let mut result = String::new();
    for x in a {
        result.push_str(&format!("{}\n", x.1));
    }
    return result.trim().to_string();
}

fn generate_sort_key(b: &Vec<usize>, mut a: usize) -> usize {
    let mut sort_key: usize = 0;
    let mut mul = 1;
    while a > 0 {
        let m = a % 10;
        sort_key += b[m] * mul;
        mul *= 10;
        a /= 10;
    }
    return sort_key;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"0 8 1 3 5 4 9 7 6 2
10
1
2
3
4
5
6
7
8
9
10",
            r"8
1
3
5
4
9
7
6
2
10"
        ),
        (
            r"0 9 8 7 6 5 4 3 2 1
3
13467932
98738462
74392",
            r"74392
98738462
13467932"
        ),
        (
            r"0 1 2 3 4 5 6 7 8 9
4
643
1234
43
909",
            r"43
643
909
1234"
        ),
        (
            r"0 7 4 3 9 5 6 2 1 8
2
333
333",
            r"333
333"
        ),
        (
            r"0 2 4 6 8 1 3 5 7 9
1
10",
            r"10"
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