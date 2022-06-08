fn main() {
    let n1 = 200u8;
    let n2 = 3u8;

    // 検査付き乗算
    assert_eq!(n1.checked_mul(n2), None);
    // 飽和乗算
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);
    // ラッピング乗算
    assert_eq!(n1.wrapping_mul(n2), 88);
    // 桁あふれ乗算
    assert_eq!(n1.overflowing_mul(n2), (88, true));
}
