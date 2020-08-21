pub fn run() {
    greeting("Howdy", "Evan");

    // Bind Fn Values to Variables
    let get_sum = add(5, 7);
    println!("Sum: {}", get_sum);

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure Sum: {}", add_nums(3, 3));

    // Closure w/ nonlocal Variable
    let n3 = 10;
    let add_nums_plus_n3 = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure NonLocal Sum: {}", add_nums_plus_n3(3, 3));

    // Swapping Pointers
    let mut a: i32 = 4;
    let mut b: i32 = 6;
    println!("a: {} -- b: {}", a, b);

    swap(&mut a, &mut b);
    println!("a: {} -- b: {}", a, b);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

fn swap(n1: &mut i32, n2: &mut i32) {
    use std::ptr;

    let x: *mut i32 = n1;
    let y: *mut i32 = n2;

    unsafe {
        ptr::swap(x, y);
    }
}
