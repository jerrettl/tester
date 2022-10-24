fn main() {
    println!("The max of 3 and 5 is {}", find_max(3, 5));

    println!("Hello, world!");

    for i in 1..30 {
        println!("i = {i}");
    }
}

fn find_max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
