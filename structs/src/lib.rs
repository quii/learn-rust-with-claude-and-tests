pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub fn perimeter(r: Rectangle) -> f64 {
    2.0 * (r.width + r.height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(perimeter(r), 30.0);
    }
}
