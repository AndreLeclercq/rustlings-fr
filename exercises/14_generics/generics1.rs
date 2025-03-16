// `Vec<T>` est générique sur le type `T` (type). Dans la plupart des cas, le compilateur est capable de
// déduire `T`, par exemple après avoir ajouté une valeur avec un type concret au vecteur.
// Mais dans cet exercice, le compilateur a besoin d'un peu d'aide via une annotation de type.

fn main() {
    // TODO: Corrige l'erreur du compilateur en annotant le type du vecteur
    // `Vec<T>`. Choisis `T` comme un type entier qui peut être créé à partir
    // de `u8` et `i8`.
    let mut numbers = Vec::new();

    // Ne change pas les lignes ci-dessous.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("Voici nos nombres : {numbers:?}");
}
