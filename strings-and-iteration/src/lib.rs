pub fn repeat(s: &str, n: usize) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push_str(s);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_a_string() {
        assert_eq!(repeat("na", 4), "nananana");
    }
}
