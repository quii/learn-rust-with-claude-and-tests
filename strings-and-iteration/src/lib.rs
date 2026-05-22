pub fn repeat(s: &str, n: usize) -> String {
    s.repeat(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_a_string() {
        assert_eq!(repeat("na", 4), "nananana");
    }
}
