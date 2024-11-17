fn main() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The counter is now: {counter}");
    }
    println!();
    for number in 0..3 {
        println!("The number is {number}");
    }
    println!();
    for number in 0..=3 {
        println!("The next number is {number}");
    }
    println!();
    for _ in 0..3 {
        println!("Printing the same thing three times");
    }
}