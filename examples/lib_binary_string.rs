fn binary_string(n: usize, keta: usize) -> String {
    return format!("{:0>1$b}", n, keta);
}

#[test]
fn test() {
    assert_eq!(binary_string(2, 3), "010");
    assert_eq!(binary_string(3, 3), "011");
}
