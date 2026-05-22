fn main() {
    println!("{}", greet(None));
}

fn greet(name: Option<&str>) -> String {
    let name = name.unwrap_or("World");
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_world_by_default() {
        assert_eq!(greet(None), "Hello, World!");
    }

    #[test]
    fn greets_a_person_by_name() {
        assert_eq!(greet(Some("Alice")), "Hello, Alice!");
    }
}
