fn main() {
    let mut message = "hello world";
    message = "sike";
    println!("{}", message);

    // mut applies to any type

    // rust allows another way to change variable by "shadowing" (remember gatsby)
    // basically just reinitializing the variable you want to mutate
    let age = 28;
    println!("{}", age);
    let age = 29;
    println!("{}", age);

    // shadowing newer but mut more common
}
