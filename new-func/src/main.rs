fn main() {
    let message = "howdy";
    println!("{}", reply(message))
}

fn reply(text: &str) -> &str {
    println!("{}", text);
    let response = "hola";
    // return response;
    // you can explicitly return as shown above, or just leave a value at the end of the function
    response
}
