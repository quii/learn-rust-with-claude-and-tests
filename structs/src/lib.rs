pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(r.perimeter(), 30.0);
    }

    #[test]
    fn area_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(r.area(), 50.0);
    }
}
