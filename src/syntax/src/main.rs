fn main() {
    let mut n: u64 = 100;
    let a: &u64 = &n;

    // Display dereference value and its address.
    // {:p} make it possible to display address
    println!("*a = {}, arrt = {:p}", *a, a);

    let b: &mut u64 = &mut n;
    *b = 200;
    println!("n = {n}");

    do_it(add, 10, 2);
    do_it(mul, 10, 2);
}

fn do_it(f: fn(u32, u32) -> u32, a: u32, b: u32) {
    println!("{}", f(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}
