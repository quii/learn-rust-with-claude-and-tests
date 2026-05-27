use std::f64::consts::PI;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

impl Rectangle {
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 10.0 };
        assert_eq!(r.perimeter(), 40.0);
    }

    #[test]
    fn area_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 10.0 };
        assert_eq!(r.area(), 100.0);
    }

    #[test]
    fn area_of_circle() {
        let c = Circle { radius: 10.0 };
        assert_eq!(c.area(), 314.1592653589793);
    }
}
