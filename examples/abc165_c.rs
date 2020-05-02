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

    let mut queries: Vec<Vec<usize>> = vec![vec![0; 4]; q];
    for i in 0..q {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let d: usize = iterator.next().unwrap().parse().unwrap();
        queries[i][0] = a;
        queries[i][1] = b;
        queries[i][2] = c;
        queries[i][3] = d;
    }

    let mut ans: usize = 0;

    let mut arrays: Vec<Vec<usize>> = Vec::new();
    let mut buf: Vec<usize> = vec![0; n];
    dfs(0, n, 1, m, &mut buf, &mut arrays);

    for array in arrays.iter() {
        let mut point: usize = 0;
        for query in queries.iter() {
            if array[query[1] - 1] - array[query[0] - 1] == query[2] {
                point += query[3];
            }
        }
        ans = max(ans, point);
    }

    return ans.to_string()
}

fn dfs(i: usize, n: usize, start: usize, end: usize, buf: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if i == n {
        result.push(buf.clone());
        return;
    } else {
        for j in start..=end {
            buf[i] = j;
            dfs(i + 1, n, j, end, buf, result);
        }
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
            r"9 10 1
1 9 8 1",
            "1"
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