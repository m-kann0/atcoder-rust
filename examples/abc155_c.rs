use std::io::Read;
use std::collections::HashMap;
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

    let mut map: HashMap<&str, usize> = HashMap::with_capacity(n);
    for _i in 0..n {
        *map.entry(iterator.next().unwrap()).or_insert(0) += 1;
    }

    let mut max_count: usize = 0;
    for (_, &v) in &map {
        max_count = max(max_count, v);
    }

    let filtered = map.into_iter()
        .filter(|&(_, v)| v == max_count)
        .collect::<HashMap<&str, usize>>();

    let mut keys = Vec::with_capacity(filtered.len());
    for &key in filtered.keys() {
        keys.push(key)
    }

    keys.sort();

    return keys.join("\n");
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"7
beat
vet
beet
bed
vet
bet
beet"
        ),
        "beet
vet"
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
