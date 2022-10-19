#[allow(dead_code)]

pub fn example() {
    // Condicionais
    let speed = 7;
    if speed < 10 {
        println!("walking 👟");
    } else if speed < 20 {
        println!("running 🏃");
    } else {
        println!("flying 🦅");
    }

    // Resultado de um jogo
    let (cor, fla) = (2, 1);
    let my_humor = if cor > fla { "euphoric" } else { "depressed" };
    println!("I'll feel {} after the game", my_humor);

    // LOOPS

    // while

    let mut i = 0;
    while i < 10 {
        print!("🦀");
        i += 1;
    }
    println!("");

    // for

    for _ in 0..10 {
        print!("🔥");
    }
    println!("");

    // for com iterador
    let animals = ["🐶", "🐱", "🐭", "🐹"];
    for (index, animal) in animals.iter().enumerate() {
        print!("{}: {} ", index + 1, animal);
    }
    println!("");

    // loop

    let mut j = 0;
    let os = ["🐧", "🍎", "🪟"];
    loop {
        if j == os.len() {
            break;
        }
        if os[j] == "🐧" {
            println!("I use Arch btw");
            break;
        }
        j += 1;
    }

    // return de loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("O resultado é {}", result);
}
