// `Vec<T>` est générique sur le type `T`. Dans la plupart des cas, le compilateur est capable de
// déduire `T`, par exemple après avoir ajouté une valeur avec un type concret au vecteur.
// Mais dans cet exercice, le compilateur a besoin d'un petit coup de pouce sous forme d'une annotation de type.

fn main() {
    // `u8` et `i8` peuvent tous deux être convertis en `i16`.
    let mut numbers: Vec<i16> = Vec::new();
    //             ^^^^^^^^^^ ajouté

    // Ne change pas les lignes ci-dessous.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
