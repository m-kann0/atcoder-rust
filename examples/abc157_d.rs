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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut friends: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _i in 0..m {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        friends[a - 1].push(b - 1);
        friends[b - 1].push(a - 1);
    }


    let mut blocks: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _i in 0..k {
        let c: usize = iterator.next().unwrap().parse().unwrap();
        let d: usize = iterator.next().unwrap().parse().unwrap();
        blocks[c - 1].push(d - 1);
        blocks[d - 1].push(c - 1);
    }


    let mut colors: Vec<usize> = vec![0; n];
    assign_color(n, &mut colors, &friends);

    let mut ans = String::new();
    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            if friends[i].contains(&j) {
                continue;
            }
            if blocks[i].contains(&j) {
                continue;
            }
            if colors[i] == colors[j] {
                count += 1;
            }
        }
        ans = format!("{} {}", ans, count);
    }

    return ans.trim().to_string();
}

fn assign_color(n: usize, colors: &mut Vec<usize>, friends: &Vec<Vec<usize>>) {
    let mut id: usize = 1;
    for i in 0..n {
        if colors[i] == 0 {
            dfs(i, id, colors, friends);
            id += 1;
        }
    }
}

fn dfs(r: usize, c: usize, colors: &mut Vec<usize>, friends: &Vec<Vec<usize>>) {
    let mut s: Vec<usize> = Vec::new();
    s.push(r);
    colors[r] = c;
    while !s.is_empty() {
        let u = s.pop().unwrap();
        for i in 0..friends[u].len() {
            let v = friends[u][i];
            if colors[v] == 0 {
                colors[v] = c;
                s.push(v);
            }
        }
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 4 1
2 1
1 3
3 2
3 4
4 1",
            "0 1 0 1"
        ),
        (
            r"5 10 0
1 2
1 3
1 4
1 5
3 2
2 4
2 5
4 3
5 3
4 5",
            "0 0 0 0 0"
        ),
        (
            r"10 9 3
10 1
6 7
8 2
2 5
8 4
7 3
10 9
6 4
5 8
2 6
7 5
3 1",
            "1 3 5 4 3 3 3 3 1 0"
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