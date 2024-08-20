#[derive(Debug)]
enum LinkedListTest {
    Cons(String, Box<LinkedListTest>),
    End,
}

impl LinkedListTest {
    fn new() -> Self {
        LinkedListTest::End
    }

    fn add_linkedlist(self, val: &str) -> Self {
        LinkedListTest::Cons(val.to_string(), Box::new(self))
    }

    fn to_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        let mut current = self;
        while let LinkedListTest::Cons(ref value, ref next) = *current {
            // println!("in loop -> {:?}", current);
            vec.push(value.to_owned());
            current = next;
        }
        vec
    }
}

// Closure impl //////////////////////////////////////////

fn fun(i: i32) -> i32 {
    i * i
}

// closure function delegation

fn run_function<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn run_add_function<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

pub fn show_closure_box_iterator() {
    // First impl ///////////////////////////////////////////
    let l = LinkedListTest::Cons(
        "One".to_string(),
        Box::new(LinkedListTest::Cons(
            "Two".to_string(),
            Box::new(LinkedListTest::End),
        )),
    );

    println!("first impl -> {:?}", l);
    // Second impl ///////////////////////////////////////////
    let l2 = LinkedListTest::new()
        .add_linkedlist("val1")
        .add_linkedlist("val2")
        .add_linkedlist("val3");

    println!("second list impl -> {:?}", l2);
    println!("second list impl to vec -> {:?}", l2.to_vec());

    // Closure

    let f = |i: i32| i + 1;
    let x = 10;
    let y = f(x);
    println!("closure -> {}", y);
    println!("closure 2 -> {}", fun(x));

    let mut x1 = 2;
    let mut f1 = || {
        x1 = x1 * x1;
        println!("closure f1 -> x1 -> {}", x1);
    };

    f1();
    f1();
    f1();

    // closure function delegation

    let local_fn = || {
        println!("hello from run function");
    };
    run_function(local_fn);

    let local_delegated_fn_addition = |i: i32| i + 10;
    println!(
        "delegated funtion -> {}",
        run_add_function(local_delegated_fn_addition)
    );

    let data = vec![1, 1, 2, 2, 3, 3, 4, 8, 9, 10];
    println!(
        "data -> {:?}",
        data.iter().take_while(|&&i| i == 2).fold(0, |s, i| s + i)
    );

    // Closure with high order
    let list_map = (0..6).map(|i| i);
    let num_list: Vec<_> = list_map.clone().collect();
    println!("high order function map -> {:?}", num_list);
    let num_list: Vec<_> = list_map.clone().take_while(|&i| i < 2).collect();
    println!("high order function take while -> {:?}", num_list);
    let num_list: Vec<_> = list_map.clone().filter(|&i| i > 2).collect();
    println!("high order function filter -> {:?}", num_list);
}
