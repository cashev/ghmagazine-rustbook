fn hello() {
    println!("Hello");
}

fn main() {
    let ret = hello();
    assert_eq!(ret, ());
}
