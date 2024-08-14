#[derive(Debug, Clone, Copy)]
struct Prime {
    number: u32,
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        // println!("In next -> Prime -> {} ", self.number);
        let mut prime_num = self.number + 1;
        if !(self.number < 3) {
            'prime_loop: loop {
                if is_prime(prime_num) {
                    break 'prime_loop;
                }
                prime_num += 1;
            }
        }
        self.number = prime_num;
        Some(prime_num)
    }
}

fn is_prime(prime_num: u32) -> bool {
    let mut count: u32 = 2;
    while count < prime_num {
        // println!(" {} - {} - {} ", prime_num, count, prime_num % count);
        // thread::sleep(Duration::from_secs(1));
        // number is dividable
        if prime_num % count == 0 {
            return false;
        }
        count += 1;
    }
    return true;
}

pub(crate) fn show_iterator_example() {
    let mut prime = Prime { number: 0 };

    for i in prime.take(14).enumerate() {
        println!("Prime number at {}  is {}", i.0, i.1);
    }

    for i in prime.skip(14).take(5).enumerate() {
        println!("Prime number at {}  is {}", i.0, i.1);
    }

    println!("Prime number is {:?}", prime.next());
    println!("Prime number is {:?}", prime.next());
    println!("Prime number is {:?}", prime.next());
    println!("Prime number is {:?}", prime.next());
    println!("Prime number is {:?}", prime.next());
}
