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
    let mut values: Vec<usize> = Vec::with_capacity(n);
    for _i in 0..n {
        values.push(iterator.next().unwrap().parse().unwrap());
    }

    values.sort_by(|a,b| b.cmp(a));
    let mut ans: f64 = values.pop().unwrap() as f64;
    for _i in 0..(n-1) {
        ans = (ans + values.pop().unwrap() as f64) / 2.0;
    }

    return format!("{:.10}", ans);
}

#[test]
fn test() {
    println!(
        "{}",
        solve(
            r"2
3 4"
        )
    );
    println!(
        "{}",
        solve(
            r"3
500 300 200"
        )
    );
    println!(
        "{}",
        solve(
            r"5
138 138 138 138 138"
        )
    );
}
