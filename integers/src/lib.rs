/// Adds two integers together.
///
/// # Examples
///
/// ```
/// assert_eq!(integers::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_integers() {
        assert_eq!(add(2, 2), 4);
    }
}
