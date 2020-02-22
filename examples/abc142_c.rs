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
    let mut vec: Vec<(usize, usize)> = Vec::with_capacity(n);
    for i in 1..(n+1) {
        vec.push((i, iterator.next().unwrap().parse().unwrap()));
    }

    vec.sort_by(|a, b| (a.1).cmp(&b.1));
    let ans: Vec<String> = vec.iter().map(|it| it.0.to_string()).collect();
    return ans.connect(" ");
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"3
2 3 1"
        ),
        "3 1 2"
    );
//    assert_eq!(
//        solve(
//            r""
//        ),
//        ""
//    );
//    assert_eq!(
//        solve(
//            r""
//        ),
//        ""
//    );
}
