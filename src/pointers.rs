/*
 *Pointers: point to ref address in memory.
 * With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a ref (&) to point to the resource.
*/
pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Arr Values: {:?}", (arr1, arr2));

    // Vector (Non-primitive)
    let vec1: *const Vec<i32> = &vec![1, 2, 3];
    let vec2 = vec1;

    println!("Vec Values: {:?}", (&vec1, &vec2));

    // Test
    let a: i8 = 123;
    let b: *const i8 = &a;
    let c: &*const i8 = &b;

    println!("b: {:p}, c: {:p}, &b: {:p}", b, c, &b);
}
