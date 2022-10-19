#[allow(dead_code)]
#[derive(Debug)]
enum Food {
    CordonBleu,
    Sushi,
    Pasta,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Country {
    Japan,
    Italy,
    Switzerland,
    Brazil,
}

#[allow(dead_code)]
pub fn example() {
    let country = Country::Brazil;

    let lunch = match country {
        Country::Japan => Food::Sushi,
        Country::Italy => Food::Pasta,
        Country::Switzerland => Food::CordonBleu,
        _ => Food::Pasta,
    };
    println!("I'll have a {:?}", lunch);
}
