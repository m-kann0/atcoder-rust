use std::io::Read;
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
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    let mut c: Vec<usize> = Vec::new();
    let mut d: Vec<usize> = Vec::new();
    for _ in 0..q {
        a.push(iterator.next().unwrap().parse().unwrap());
        b.push(iterator.next().unwrap().parse().unwrap());
        c.push(iterator.next().unwrap().parse().unwrap());
        d.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut array: Vec<usize> = vec![0; n];
    let mut arrays: Vec<Vec<usize>> = Vec::new();
    dfs(0, n, m, &mut array, &mut arrays);

    let mut ans: usize = 0;
    for array in arrays.iter() {
        let mut score: usize = 0;
        for i in 0..q {
            if array[b[i] - 1] - array[a[i] - 1] == c[i] {
                score += d[i];
            }
        }
        ans = max(ans, score);
    }

    return ans.to_string()
}

fn dfs(i: usize, n: usize, m: usize, array: &mut Vec<usize>, arrays: &mut Vec<Vec<usize>>) {
    if i == n {
        arrays.push(array.clone());
        return;
    }

    let mut last: usize = if i == 0 {
        1
    } else {
        array[i - 1]
    };
    while last <= m {
        array[i] = last;
        dfs(i + 1, n, m, array, arrays);
        last += 1;
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4 3
1 3 3 100
1 2 2 10
2 3 2 10",
            "110"
        ),
        (
            r"4 6 10
2 4 1 86568
1 4 0 90629
2 3 0 90310
3 4 1 29211
3 4 3 78537
3 4 2 8580
1 2 1 96263
1 4 2 2156
1 2 0 94325
1 4 3 94328",
            "357500"
        ),
        (
            r"10 10 1
1 10 9 1",
            "1"
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