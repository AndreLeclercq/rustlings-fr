// Ce programme crée plusieurs threads (fils d'exécution) qui tournent chacun pendant au moins 250ms, et
// chaque thread retourne le temps qu'il a mis pour se terminer. Le programme doit
// attendre que tous les threads créés aient fini et collecter leurs
// valeurs de retour dans un vecteur.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} terminé");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: Collecte les résultats de tous les threads dans le vecteur `results`.
        // Utilise la struct `JoinHandle` qui est retournée par `thread::spawn`.
    }

    if results.len() != 10 {
        panic!("Oh non ! Certains threads ne sont pas encore terminés !");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Le thread {i} a pris {result}ms");
    }
}
