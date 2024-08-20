mod testing_custom_iterator;
mod testing_enum;
mod testing_generics;
mod testing_operator_overload_struct;
mod testing_structs;
mod testing_traits;
mod closure_box_iterator;
extern crate text_io;

use testing_custom_iterator::show_iterator_example;
use testing_generics::show_generic_example;
use testing_operator_overload_struct::add_operator_on_struct;
use testing_traits::trait_impl_demo;
use closure_box_iterator::show_closure_box_iterator;

use crate::testing_enum::UnitOperation;
use crate::testing_enum::Units;
use crate::testing_enum::Vehicle;
use crate::testing_structs::Circle;
use crate::text_io::scan;

fn main() {
    let scn: i32;
    println!(
        "Enter a number to execute example 
        1. Struct example 
        2. Match and loop example
        3. Enums
        4. Traits
        5. Operator overload on struct
        6. custom iterator
        7. Generics example
        8. Closure , Box and iterator
        "
    );
    scan!("{}", scn);

    match scn {
        1 => struct_example(),
        2 => do_match_and_loop(),
        3 => run_enum_example(),
        4 => run_trait_example(),
        5 => run_operator_overload_struct_example(),
        6 => run_custom_iterator_example(),
        7 => run_generics_example(),
        8 => run_closure_box_iterator_exmpl(),
        _ => println!("Nothing to process"),
    }
}

fn run_closure_box_iterator_exmpl() {
    show_closure_box_iterator()
}

fn run_generics_example() {
    show_generic_example()
}

fn run_custom_iterator_example() {
    show_iterator_example();
}

fn run_operator_overload_struct_example() {
    add_operator_on_struct();
}

fn run_trait_example() {
    trait_impl_demo();
}

fn run_enum_example() {
    let exmpl1 = UnitOperation::Add(Units::new(7, 8));
    let exmpl2 = UnitOperation::Subtract(Units::new(7, 8));
    let exmpl3 = UnitOperation::Multiply(Units::new(7, 8));
    let exmpl4 = UnitOperation::Devide(Units::new(1, 0));
    exmpl1.do_operation();
    exmpl2.do_operation();
    exmpl3.do_operation();
    exmpl4.do_operation();

    let vehicle1 = Vehicle::Bicycle { tyre: 2 };
    let vehicle2 = Vehicle::MotorBike {
        tyre: 2,
        two_stroke_engine: true,
    };
    let vehicle3 = Vehicle::Car {
        tyre: 4,
        four_stroke_engine: true,
        pessenger: 4,
    };
    vehicle1.show_me();
    vehicle2.show_me();
    vehicle3.show_me();
    println!("----------------- 3. over --------------------");
}

fn struct_example() {
    println!("1. Struct and Trait example");
    let c: Circle = Circle::new(4);
    c.show();

    println!("Circle -> {:?}", &c);
    println!("Pretty Circle -> {:#?}", &c);

    println!("{}", c);

    println!("----------------- 1. over --------------------");
}

fn do_match_and_loop() {
    println!("2. match example");
    let i = 5;
    let mut j = 0;
    loop {
        println!("{}", j);
        if j == i {
            break;
        }
        j += 1;
    }
    let p = match i {
        n @ 0..=5 => {
            println!("match -> {}", n);
            1
        }
        n @ 6..=10 => {
            println!("match -> {}", n);
            2
        }
        n @ _ => {
            println!("match -> {}", n);
            2
        }
    };

    println!("p -> {}", p);
    println!("----------------- 2. over --------------------");
}
