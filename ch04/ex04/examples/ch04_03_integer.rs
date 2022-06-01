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
}
