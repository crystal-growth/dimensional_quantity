#[cfg(feature = "std")]
pub (crate) fn dim_to_string(i: i64) -> Option<String> {
    if i == 0 {
        None
    } else if i == 1 {
        Some("".into())
    } else {
        Some(i.to_string()
            .replace("0", "⁰")
            .replace("1", "¹")
            .replace("2", "²")
            .replace("3", "³")
            .replace("4", "⁴")
            .replace("5", "⁵")
            .replace("6", "⁶")
            .replace("7", "⁷")
            .replace("8", "⁸")
            .replace("9", "⁹")
            .replace("-", "⁻"))
    }
}