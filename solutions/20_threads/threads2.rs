// En s'appuyant sur l'exercice précédent, nous voulons que tous les threads terminent leur
// travail. Mais cette fois, les threads lancés doivent être responsables de la mise à jour d'une
// valeur partagée : `JobStatus.jobs_done`

use std::{
   sync::{Arc, Mutex},
   thread,
   time::Duration,
};

struct JobStatus {
   jobs_done: u32,
}

fn main() {
   // `Arc` n'est pas suffisant si tu veux un état partagé **mutable**.
   // Nous devons envelopper la valeur avec un `Mutex` (verrou d'exclusion mutuelle).
   let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
   //                    ^^^^^^^^^^^                          ^

   let mut handles = Vec::new();
   for _ in 0..10 {
       let status_shared = Arc::clone(&status);
       let handle = thread::spawn(move || {
           thread::sleep(Duration::from_millis(250));

           // Verrouille avant de mettre à jour une valeur partagée.
           status_shared.lock().unwrap().jobs_done += 1;
           //           ^^^^^^^^^^^^^^^^
       });
       handles.push(handle);
   }

   // Attente que tous les travaux soient terminés.
   for handle in handles {
       handle.join().unwrap();
   }

   println!("Travaux terminés : {}", status.lock().unwrap().jobs_done);
   //                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
