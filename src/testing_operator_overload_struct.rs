use std::ops;

struct One {
    x: i32,
    y: i32,
}

struct Two {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Third {
    x: i32,
    y: i32,
}

impl ops::Add<One> for Two {
    type Output = Third;

    fn add(self, rhs: One) -> Third {
        Third {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub(crate) fn add_operator_on_struct() {
    let one = One { x: 1, y: 1 };
    let two = Two { x: 2, y: 2 };
    let third = two + one;
    println!("one + two -> third : ( {} {} )", third.x, third.y);
}
