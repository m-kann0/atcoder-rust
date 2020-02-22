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
    let k: i64 = iterator.next().unwrap().parse().unwrap();
    let q: i64 = iterator.next().unwrap().parse().unwrap();

    let mut map: HashMap<usize, i64> = HashMap::new();
    for _i in 0..q {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        *map.entry(a).or_insert(0) += 1;
    }

    let mut answer = Vec::with_capacity(n);
    for i in 1..(n + 1) {
        if k - q + *map.get(&i).unwrap_or(&0) > 0 {
            answer.push("Yes");
        } else {
            answer.push("No");
        }
    }

    return answer.join("\n");
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"6 3 4
3
1
3
2"
        ),
        "No
No
Yes
No
No
No"
    );
    assert_eq!(
        solve(
            r"6 5 4
3
1
3
2"
        ),
        "Yes
Yes
Yes
Yes
Yes
Yes"
    );
    assert_eq!(
        solve(
            r"10 13 15
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
9"
        ),
        "No
No
No
No
Yes
No
No
No
Yes
No"
    );
}
