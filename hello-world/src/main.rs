fn main() {
    println!("{}", greet("World"));
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }


}
