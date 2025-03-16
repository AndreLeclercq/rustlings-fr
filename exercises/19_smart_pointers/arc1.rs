// Dans cet exercice, on te donne un `Vec` (vecteur) de `u32` appelé `numbers` avec des valeurs
// allant de 0 à 99. Tu voudrais utiliser cet ensemble de nombres dans 8
// threads différents simultanément. Chaque thread va calculer la somme
// de chaque huitième valeur avec un décalage (offset).
//
// Le premier thread (décalage 0), va sommer 0, 8, 16, …
// Le deuxième thread (décalage 1), va sommer 1, 9, 17, …
// Le troisième thread (décalage 2), va sommer 2, 10, 18, …
// …
// Le huitième thread (décalage 7), va sommer 7, 15, 23, …
//
// Chaque thread doit posséder un pointeur de comptage de références vers le vecteur de
// nombres. Mais `Rc` n'est pas thread-safe (sûr pour les threads). Donc, on a besoin d'utiliser `Arc`.
//
// Ne te laisse pas distraire par la façon dont les threads sont créés et rejoints. 
// On s'entraînera à cela plus tard dans les exercices sur les threads.

// Ne change pas les lignes ci-dessous.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: Définis `shared_numbers` en utilisant `Arc`.
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: Définis `child_numbers` en utilisant `shared_numbers`.
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Somme du décalage {offset} est {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
