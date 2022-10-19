struct Student {
    name: String,
    year_joined: u32,
}

#[allow(dead_code)]
fn can_graduate(student: &Student) -> bool {
    let since_1970_in_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let current_year = 1970 + since_1970_in_secs / 31536000;
    current_year - student.year_joined >= 4
}

fn drop_student(student: Student) {
    println!("{} graduated!", student.name);
}

#[allow(dead_code)]
pub fn example() {
    let student = Student {
        name: String::from("Gustavo Becelli"),
        year_joined: 2020,
    };

    let student_can_graduate = can_graduate(&student);
    println!("{} can graduate: {}", student.name, student_can_graduate);

    drop_student(student);

    // Erro: o estudante não pode ser usado porque foi movido e não existe mais
    // println!("{} can graduate: {}", student.name, student_can_graduate);
}
