fn diz_info(data: (String, u32)) {
    let (nome, ano_nascimento) = data;
    println!("Olá, {}, nascido em {}.", nome, ano_nascimento);
}

fn main() {
    let minha_tupla = (String::from("Alucard"), 1456);
    diz_info(minha_tupla);
    diz_info(minha_tupla);
}


