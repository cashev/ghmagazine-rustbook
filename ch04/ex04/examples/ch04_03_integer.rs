use std::any::type_name;

fn type_of<T>(_: T) -> String {
    let a = type_name::<T>();
    a.to_string()
}

fn main() {
    let n1 = 10_000;
    let n2 = 0u8;
    let n3 = -100_isize;

    let n4 = 10;
    let n5 = n3 + n4;
    println!("n1: {}", type_of(n1));
    println!("n2: {}", type_of(n2));
    println!("n3: {}", type_of(n3));
    println!("n4: {}", type_of(n4));
    println!("n5: {}", type_of(n5));

    let h1 = 0xff;
    let o1 = 0o744;
    let b1 = 0b1010_0110_1110_1001;
    println!("h1: {}, val: {}", type_of(h1), h1);
    println!("o1: {}, val: {}", type_of(o1), o1);
    println!("b1: {}, val: {}", type_of(b1), b1);

    let n6 = b'A';
    assert_eq!(n6, 65u8);
    println!("n6: {}, val: {}", type_of(n6), n6);
}
