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

    let mut i: usize = 2;
    while i * i <= n {
        if n % i == 0 {
            return "NO".to_string();
        }
        i += 1;
    }

    return "YES".to_string();
}
