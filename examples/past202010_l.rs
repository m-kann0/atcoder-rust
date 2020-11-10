use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let mut h: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut map: HashMap<isize, usize> = HashMap::new();
    for i in 1..n {
        let k = if i % 2 == 1 {
            h[i] - h[i - 1]
        } else {
            h[i - 1] - h[i]
        };
        *map.entry(k).or_insert(0) += 1;
    }

    let mut result = String::new();
    let mut x: isize = 0;
    for _ in 0..q {
        let t: usize = iterator.next().unwrap().parse().unwrap();
        if t == 1 {
            let v: isize = iterator.next().unwrap().parse().unwrap();
            x += v;
        } else if t == 2 {
            let v: isize = iterator.next().unwrap().parse().unwrap();
            x -= v;
        } else {
            let u: usize = iterator.next().unwrap().parse().unwrap();
            let v: isize = iterator.next().unwrap().parse().unwrap();
            let u = u - 1;
            if u == 0 {
                let old_key = h[1] - h[0];
                *map.entry(old_key).or_insert(0) -= 1;
                h[0] += v;
                let new_key = h[1] - h[0];
                *map.entry(new_key).or_insert(0) += 1;
            } else if u == n - 1 {
                let k = if u % 2 == 1 {
                    h[u] - h[u - 1]
                } else {
                    h[u - 1] - h[u]
                };
                *map.entry(k).or_insert(0) -= 1;
                h[u] += v;
                let k = if u % 2 == 1 {
                    h[u] - h[u - 1]
                } else {
                    h[u - 1] - h[u]
                };
                *map.entry(k).or_insert(0) += 1;
            } else {
                for i in u..=(u + 1) {
                    let k = if i % 2 == 1 {
                        h[i] - h[i - 1]
                    } else {
                        h[i - 1] - h[i]
                    };
                    *map.entry(k).or_insert(0) -= 1;
                }
                h[u] += v;
                for i in u..=(u + 1) {
                    let k = if i % 2 == 1 {
                        h[i] - h[i - 1]
                    } else {
                        h[i - 1] - h[i]
                    };
                    *map.entry(k).or_insert(0) += 1;
                }
            }
        }
        let zero = 0;
        result.push_str(&format!("{}\n", map.get(&x).unwrap_or(&zero)));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 2
10 20 30 20
1 10
3 4 20",
            "1
2"
        ),
        (
            r"10 20
108 112 112 110 110 109 108 110 111 112
3 4 2
1 1
3 9 1
3 4 2
2 1
1 1
3 7 2
2 1
3 8 2
1 1
2 1
1 1
3 10 2
3 6 1
2 1
3 7 1
1 1
2 1
3 3 2
3 3 2",
            "2
2
1
1
2
1
0
3
3
0
3
0
0
0
4
3
1
3
3
2"
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