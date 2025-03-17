use std::mem;

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // L'`unwrap` d'une `Option` après avoir vérifié si elle est `None` provoquera un panic.
    // Utilise `if-let` à la place.
    if let Some(value) = my_option {
        println!("{value}");
    }

    // Une virgule manquait.
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("Mon tableau ! Le voici : {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // `resize` modifie un vecteur au lieu d'en retourner un nouveau.
    // `resize(0, …)` vide un vecteur, il est donc préférable d'utiliser `clear`.
    my_empty_vec.clear();
    println!("Ce Vec est vide, tu vois ? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // Utilise `mem::swap` pour échanger correctement deux valeurs.
    mem::swap(&mut value_a, &mut value_b);
    println!("valeur a : {}; valeur b : {}", value_a, value_b);
}
