#[allow(dead_code)]
pub fn example() {
    // Vetor mutável
    let mut v = vec![1, 2, 3, 4, 5];
    // Slice das posições 1 a 3
    let slice = &mut v[1..3];

    // Alterando o slice
    slice[0] = 10;

    // Se a slice mutável for acessada na linha X, não é possível
    // refenciar o vetor nas linhas anteriores a X.
    // println!("{:?}", v);
    // slice[1] = 20;

    // É possível dereferenciar o slice mutável.
    // drop(slice);
    // println!("{:?}", &slice); //Erro: slice não existe mais
}
