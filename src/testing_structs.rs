use std::f64::consts::PI;
use std::fmt;

#[derive(Debug)]

pub struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "From OBJ \n Radius : {} \n Parameter : {} \n Area {}",
            self.radius,
            self.parameter(),
            self.area(),
        )
    }
}

impl Circle {
    pub fn show(&self) {
        println!(
            "Radius : {} and Parameter : {} and Area {}",
            self.radius,
            self.parameter(),
            self.area()
        )
    }
}

impl Circle {
    pub fn new(radius: i32) -> Circle {
        Circle { radius }
    }

    pub fn parameter(&self) -> f64 {
        2.0 * PI * self.radius as f64
    }

    pub fn area(&self) -> f64 {
        PI * self.radius.pow(2) as f64
    }
}
