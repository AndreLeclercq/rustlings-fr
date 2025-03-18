#![allow(clippy::needless_late_init)]

fn main() {
    // Lire des variables non initialisées n'est pas autorisé en Rust !
    // Par conséquent, nous devons d'abord assigner une valeur.
    let x: i32 = 42;

    println!("Nombre {x}");

    // Il est possible de déclarer une variable et de l'initialiser plus tard.
    // Mais elle ne peut pas être utilisée avant l'initialisation.
    let y: i32;
    y = 42;
    println!("Nombre {y}");
}
