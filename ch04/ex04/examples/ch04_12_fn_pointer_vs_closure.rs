fn main() {
    let x = 4;

    let adder = |n| n + x;
    assert_eq!(adder(2), 4 + 2);

    let mut state = false;
    let mut flipflop = || {
        state = !state;
        state
    };

    assert!(flipflop());
    assert!(!flipflop());
    assert!(flipflop());

    assert!(state);
}
