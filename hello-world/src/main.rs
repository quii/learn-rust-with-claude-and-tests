fn main() {
    println!("{}", greet("World"));
}

fn greet(name: &str) -> String {
    if name.is_empty() {
        format!("Hello, World!")
    } else {
        format!("Hello, {}!", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_world_by_default() {
        assert_eq!(greet(""), "Hello, World!");
    }

    #[test]
    fn greets_a_person_by_name() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
