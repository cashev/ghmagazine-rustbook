use std::any::type_name;

fn type_of<T>(_: T) -> String {
    let a = type_name::<T>();
    a.to_string()
}

fn double(n: i32) -> i32 {
    n + n
}

fn abs(n: i32) -> i32 {
    if n >= 0 {
        n
    } else {
        -n
    }
}

fn main() {
    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

    f = abs;
    assert_eq!(f(42), 42);

    let mut f_bad = double;
    println!("f_bad: {}", type_of(f_bad));
    println!("abs: {}", type_of(abs));
    assert_eq!(std::mem::size_of_val(&f_bad), 0);
}
