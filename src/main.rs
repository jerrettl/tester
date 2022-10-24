fn main() {
    println!("The max of 3 and 5 is {}", find_max(3, 5));

    println!("Hello, world!");

    println!("This is missing quotes");
}

fn find_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
