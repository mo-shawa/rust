fn main() {
    let _float_num: f32 = 3.14;

    // tuples
    let _tup: (i32, &str, f32) = (20, "string kek", 1.0000003);
    // you can access tuple at an index using dot notation
    println!("{}", _tup.1);

    // you can destructure tuples into variables, but the number of parameters needs to match the original:
    let (integer, string, floater) = _tup;
    println!("{} {} {}", integer, string, floater);

    // arrays:
    // array types are represented with a literal containing the contained type and the number of els
    // e.g. []
    let arr = [1, 2, 3, 4, 5];
    println!("for in loop (element):");

    for el in arr {
        println!("{}", el)
    }

    println!("for in loop (iterator):");
    for i in 0..arr.len() {
        println!("{}", arr[i])
    }
}
