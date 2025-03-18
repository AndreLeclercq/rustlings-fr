fn square(num: i32) -> i32 {
    // Suppression du point-virgule `;` à la fin de la ligne ci-dessous pour retourner implicitement le résultat.
    num * num
}

fn main() {
    let answer = square(3);
    println!("Le carré de 3 est {answer}");
}
