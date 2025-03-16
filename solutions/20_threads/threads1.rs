// Ce programme lance plusieurs threads qui s'exécutent pendant au moins 250ms, et
// chaque thread renvoie le temps qu'il a mis pour s'exécuter. Le programme doit
// attendre que tous les threads lancés soient terminés et doit collecter leurs
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
            println!("Thread (fil d'exécution) {i} terminé");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // Collecte les résultats de tous les threads dans le vecteur `results`.
        results.push(handle.join().unwrap());
    }

    if results.len() != 10 {
        panic!("Oh non ! Un thread n'est pas encore terminé !");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Le thread {i} a pris {result}ms");
    }
}
