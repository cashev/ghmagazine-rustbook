fn main() {
    let b1 = true;
    println!("b1: {}", b1);
    let b2 = !b1;
    println!("b2: {}", b2);

    let n1 = 8;
    let n2 = 12;
    let b3 = n1 > 10;
    println!("b3: {}", b3);
    let b4 = n2 >= 10;
    println!("b4: {}", b4);
    let b5 = b3 && b4;
    println!("b5: {}", b5);
    let b6 = b3 || b4;
    println!("b6: {}", b6);

    assert_eq!(std::mem::size_of::<bool>(), 1);
}