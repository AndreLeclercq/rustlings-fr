// Dans cet exercice, on nous donne un `Vec` de `u32` appelé `numbers` (nombres) avec des valeurs
// allant de 0 à 99. Nous voulons utiliser cet ensemble de nombres dans 8
// threads (fils d'exécution) différents simultanément. Chaque thread va calculer la somme de
// chaque huitième valeur avec un décalage.
//
// Le premier thread (décalage 0) additionnera 0, 8, 16, …
// Le deuxième thread (décalage 1) additionnera 1, 9, 17, …
// Le troisième thread (décalage 2) additionnera 2, 10, 18, …
// …
// Le huitième thread (décalage 7) additionnera 7, 15, 23, …
//
// Chaque thread devrait posséder un pointeur de comptage de références vers le vecteur de
// nombres. Mais `Rc` n'est pas thread-safe (sécurisé pour les threads). Par conséquent, nous devons utiliser `Arc`.
//
// Ne te laisse pas distraire par la façon dont les threads sont lancés et joints. Nous pratiquerons
// cela plus tard dans les exercices sur les threads.

// Ne change pas les lignes ci-dessous.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    let shared_numbers = Arc::new(numbers);
    //                   ^^^^^^^^^^^^^^^^^

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        //                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("La somme du décalage {offset} est {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
