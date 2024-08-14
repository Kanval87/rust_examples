use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> u32;
}

pub struct Circle {
    radius: f64,
}

pub struct Square {
    height: i32,
    width: i32,
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (PI * self.radius * self.radius) as u32
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        (self.height * self.width) as u32
    }
}

pub(crate) fn trait_impl_demo() {
    let crcl = Circle { radius: 12.0 };
    let sqr = Square {
        height: 4,
        width: 6,
    };

    println!("Trait example -> Cirle -> {} | Square -> {} ", crcl.area(), sqr.area());
}
