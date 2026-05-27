use std::f64::consts::PI;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

pub fn describe_area(shape: &dyn Shape) -> String {
    format!("This shape has an area of {}", shape.area())
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

    #[test]
    fn description_of_shapes() {
        let shapes: Vec<(&str, &dyn Shape, &str)> = vec![
            ("rectangle", &Rectangle { width: 10.0, height: 10.0 }, "This shape has an area of 100"),
            ("circle", &Circle { radius: 10.0 }, "This shape has an area of 314.1592653589793"),
            ("triangle", &Triangle { base: 5.0, height: 10.0 }, "This shape has an area of 25"),
        ];

        for (name, shape, want) in shapes {
            assert_eq!(describe_area(shape), want, "failed for {}", name);
        }
    }
}
