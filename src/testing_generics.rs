use std::{fmt::Debug, ops::Add};

// Generic Struct ////////////////////////////////////////////////////////////////////
struct GenericOne<A, B, C> {
    x: A,
    y: B,
    z: C,
}

impl<A: Debug, B: Debug, C: Debug> GenericOne<A, B, C> {
    pub(crate) fn new(x: A, y: B, z: C) -> Self {
        GenericOne { x, y, z }
    }

    fn print(&self) {
        println!("Generic One -> {:?} - {:?} - {:?}", self.x, self.y, self.z)
    }
}

// Generic Method ////////////////////////////////////////////////////////////////////
fn process_generic_function<T: Debug>(value: T) {
    println!("This is generic function -> {:?}", value);
}

// Generic Debug impl ////////////////////////////////////////////////////////////////////
enum GenericImplemention<T> {
    Val(T),
}

impl<T: Debug> Debug for GenericImplemention<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(arg0) => f.debug_tuple("Val").field(arg0).finish(),
        }
    }
}
// Generic Struct with trait impl ////////////////////////////////////////////////////////////////////

trait Convertible {
    fn show_text(&self);
}

#[derive(Debug)]
struct Transformer<U> {
    x: U,
}

impl<U> Transformer<U> {
    fn new(x: U) -> Self {
        Transformer { x }
    }
}

impl<U: Debug> Convertible for Transformer<U> {
    fn show_text(&self) {
        println!("show text -> {:?}", self.x);
    }
}

// Generic Struct with Generic trait impl ////////////////////////////////////////////////////////////////////

trait Shape<T> {
    fn area(&self) -> T;
}

struct Rectangle<T: Add> {
    x: T,
    y: T,
}

impl<T: Debug + Copy> Shape<T> for Rectangle<T>
where
    T: Add<Output = T>,
{
    fn area(&self) -> T {
        self.x + self.y
    }
}

// End impl ////////////////////////////////////////////////////////////////////
pub(crate) fn show_generic_example() {
    let generic_one = GenericOne::new(32, 64.0, "string");

    generic_one.print();

    process_generic_function("value");

    let gen_impl = GenericImplemention::Val("Gen Impl");
    println!("{:?}", gen_impl);

    let tran = Transformer::new(21);
    tran.show_text();

    let rec = Rectangle { x: 2, y: 3 };
    println!("rec area -> {}", rec.area());
}
