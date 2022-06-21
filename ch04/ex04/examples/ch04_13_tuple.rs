fn main() {
    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    assert_eq!(t1.1, true);

    let mut t1 = (88, true);
    t1.0 += 100;
    assert_eq!(t1.0, 188);
}
