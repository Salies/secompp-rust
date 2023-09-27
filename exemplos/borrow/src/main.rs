fn greet(name: &String) {
    println!("Hello, {}!", name);
}

fn goodbye(name: String) {
    println!("Goodbye, {}!", name);
}

fn main() {
    let name = String::from("Gustavo Becelli");
    greet(&name);
    greet(&name);
    goodbye(name);
    //greet(&name);
}