fn main() {
    // La façon la plus simple de corriger l'erreur du compilateur est d'initialiser
    // la variable `x`. En lui attribuant une valeur entière, Rust déduit son type
    // comme `i32` qui est le type par défaut pour les entiers.
    let x = 42;

    // Mais nous pouvons imposer un type différent du `i32` par défaut en ajoutant
    // une annotation de type:
    // let x: u8 = 42;

    if x == 10 {
        println!("x est dix !");
    } else {
        println!("x n'est pas dix !");
    }
}
