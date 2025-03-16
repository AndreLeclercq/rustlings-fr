// En continuant l'exercice précédent, nous voulons que tous les threads terminent leur
// travail. Mais cette fois, les threads créés doivent être responsables de la mise à jour
// d'une valeur partagée : `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` n'est pas suffisant si tu veux un état partagé **mutable**.
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: Tu dois prendre une action avant de mettre à jour une valeur partagée.
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Attente de la fin de tous les jobs.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Affiche la valeur de `JobStatus.jobs_done`.
    println!("Jobs terminés : {}", todo!());
}
