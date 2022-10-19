#![allow(dead_code)]
type Rgba = (u8, u8, u8, u8);
type Image = Vec<Vec<Rgba>>;
type Point = (i32, i32);
type Point3d = (i32, i32, i32);

pub fn example() {
    // Arrays
    let array = [1, 2, 3, 4, 5];
    println!("a has {} elements", array.len());
    println!("The first element of a is {}", array[0]);

    // Vectors
    let mut vector = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    // Tuples
    let tuples = (1, "hello", 4.5, true);
    println!("The value of x is: {}", tuples.0);
}
