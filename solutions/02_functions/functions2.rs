// Le type des arguments d'une fonction doit être annoté.
// Type d'annotation `u64` ajouté.
fn call_me(num: u64) {
    for i in 0..num {
        println!("Dring ! Appel numéro {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
