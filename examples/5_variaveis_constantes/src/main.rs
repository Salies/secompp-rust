/*
Rust has two different types of constants which can be declared in any scope including global. 
Both require explicit type annotation:

const: An unchangeable value (the common case).
static: A possibly mutable variable with 'static lifetime. 
The static lifetime is inferred and does not have to be specified. 
Accessing or modifying a mutable static variable is unsafe.
*/

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser

    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    //println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    //THRESHOLD = 5;
    // FIXME ^ Comment out this line

    //mutabilidade
    // Variable bindings are immutable by default, but this can be overridden using the mut modifier.
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    //_immutable_binding += 1;
    // FIXME ^ Comment out this line

    // PASSAR PRO PRÓXIMO SLIDE - ESCOPOS
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    //println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);

    // SHADOWING
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    //It's possible to declare variable bindings first, and initialize them later. 
    // However, this form is seldom used, as it may lead to the use of uninitialized variables.
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized binding
    //println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;
    //another_binding = 3; // CAUSA ERRO! explicar: "olha só que interessante"

    println!("another binding: {}", another_binding);
}
