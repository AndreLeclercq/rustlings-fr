fn main() {
    let number = "T-R-O-I-S";
    println!("Épelle un nombre : {}", number);

    // Utilisation du masquage (shadowing) de variable
    // https://jimskapt.github.io/rust-book-fr/ch03-01-variables-and-mutability.html#le-masquage
    let number = 3;
    println!("Le nombre plus deux égale : {}", number + 2);
}
