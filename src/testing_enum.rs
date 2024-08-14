#[allow(dead_code)]
#[derive(Debug)]
pub enum UnitOperation {
    Add(Units),
    Subtract(Units),
    Multiply(Units),
    Devide(Units),
}

impl UnitOperation {
    pub fn do_operation(&self) {
        match self {
            UnitOperation::Add(u) => println!("Add {}", u.num_one + u.num_two),
            UnitOperation::Subtract(u) => println!("Subtract {}", u.num_one - u.num_two),
            UnitOperation::Multiply(u) => println!("Multiply {}", u.num_one * u.num_two),
            UnitOperation::Devide(u) => {
                if u.num_two == 0 {
                    println!("Number being devided by zero")
                } else {
                    println!("Devide {}", u.num_one / u.num_two)
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Units {
    num_one: i128,
    num_two: i128,
}

impl Units {
    pub fn new(num_one: i128, num_two: i128) -> Units {
        Units { num_one, num_two }
    }
}

///////////////// - second example

pub enum Vehicle {
    Bicycle {
        tyre: u32,
    },
    MotorBike {
        tyre: u32,
        two_stroke_engine: bool,
    },
    Car {
        tyre: u32,
        four_stroke_engine: bool,
        pessenger: u32,
    },
}

impl Vehicle {
    pub fn show_me(&self) {
        match self {
            Vehicle::Bicycle { tyre } => println!("Bicycle -> tyre : {}", tyre),
            Vehicle::MotorBike {
                tyre,
                two_stroke_engine,
            } => println!(
                "MotorBike -> tyre : {} | two_stroke_engine : {}",
                tyre, two_stroke_engine
            ),
            Vehicle::Car {
                tyre,
                four_stroke_engine,
                pessenger,
            } => println!(
                "Car -> tyre : {} | four_stroke_engine : {} | pessenger : {}",
                tyre, four_stroke_engine, pessenger
            ),
        }
    }
}
