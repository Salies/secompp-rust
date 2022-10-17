fn main() {
    println!("Hello, world!");
    // comentário uma linha
    /*
    comentário multi-linha
    */
    // Formatando prints
    // Você pode colocar só {} e o Rust preenche na ordem
    println!("{} é a resposta!", 42);
    // Você pode numerar, e o Rust preenche na ordem.
    println!("{0}? não, {1}. ou seria {0} mesmo? certamente {0}...", "batata", "potato");
    // Você pode nomear os argumentos
    println!("{nome} é um {ser} muito {car}.", nome="Daniel", ser="humano", car="chato");
    println!("{nome} é um {ser} muito {car}.", nome="Zwei", ser="cachorro", car="fofo");
    // Posso formatar números de jeitos diferentes
    println!("decimal:               {}",   15);
    println!("binário:               {:b}", 15);
    println!("base 8 (octal):        {:o}", 15);
    println!("hexadecimal minúsculo: {:x}", 15);
    println!("hexadecimal maiúsculo: {:X}", 15);
    // Agora começa a loucura do Rust
    // You can right-align text with a specified width. This will output
    // "    1". 4 white spaces and a "1", for a total width of 5.
    println!("{number:>5}", number=1);
    // You can pad numbers with extra zeroes. This will output "00001".
    println!("{number:0>5}", number=1);
    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);
    // Rust even checks to make sure the correct number of arguments are
    // used.
    //println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"
    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default
    #[allow(dead_code)]
    struct Structure(i32);
    // This will not compile because `Structure` does not implement
    // fmt::Display
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
