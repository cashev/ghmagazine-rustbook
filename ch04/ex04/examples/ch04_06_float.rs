use std::any::type_name;

fn type_of<T>(_: T) -> String {
    let a = type_name::<T>();
    a.to_string()
}

fn main() {
    let f1 = 10.0;
    let f2 = -1_234.56f32;
    let f3 = 578.6E+77;

    println!("f1: {}, val: {}", type_of(f1), f1);
    println!("f2: {}, val: {}", type_of(f2), f2);
    println!("f3: {}, val: {}", type_of(f3), f3);
}
