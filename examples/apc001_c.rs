fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    println!("0");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut start = buf.trim().to_string();
    if start == "Vacant" {
        return;
    }

    let mut begin = 0;
    let mut end = n;
    for _ in 0..19 {
        let mid = (begin + end) / 2;
        println!("{}", mid);

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let current = buf.trim().to_string();
        if current == "Vacant" {
            return;
        }

        if begin + 1 == mid {
            begin = mid;
        } else if (mid - begin) % 2 == 0 && current == start || (mid - begin) % 2 == 1 && current != start {
            begin = mid;
            start = current;
        } else {
            end = mid;
        }
    }
}
