fn main() {
    // Un tableau de 100 éléments de valeur 42.
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, c'est un grand tableau !");
    } else {
        println!("Bah, je mange des tableaux comme ça au petit-déjeuner.");
        panic!("Tableau pas assez grand, plus d'éléments nécessaires");
    }
}
