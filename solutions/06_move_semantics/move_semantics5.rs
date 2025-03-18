#![allow(clippy::ptr_arg)]

// Emprunte au lieu de prendre possession.
// Il est recommandé d'utiliser `&str` au lieu de `&String` ici. Mais c'est
// suffisant pour l'instant car nous n'avons pas encore géré les strings (chaînes).
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Prend possession au lieu d'emprunter.
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust est génial!".to_string();

    get_char(&data);

    string_uppercase(data);
}
