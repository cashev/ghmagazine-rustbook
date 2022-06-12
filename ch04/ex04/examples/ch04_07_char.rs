fn main() {
    let c1 = 'A';
    let c2 = 'a';
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    let c4 = '\t';
    let c5 = '\n';
    let c6 = '\'';
    let c7 = '\\';
    let c8 = '\x7F';
    let c9 = '漢';
    let c10 = '\u{5b57}';
    let c11 = '\u{1f600}';

    println!("c4: {}", c4);
    println!("c5: {}", c5);
    println!("c6: {}", c6);
    println!("c7: {}", c7);
    println!("c8: {}", c8);
    println!("c9: {}", c9);
    println!("c10: {}", c10);
    println!("c11: {}", c11);

    assert_eq!(std::mem::size_of::<char>(), 4);
}
