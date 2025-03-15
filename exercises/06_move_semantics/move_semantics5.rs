#![allow(clippy::ptr_arg)]

// TODO: Corrige les erreurs de compilation sans rien changer sauf ajouter ou
// supprimer des références (le caractère `&`).

// Ne doit pas prendre la possession
fn get_char(data: String) -> char {
   data.chars().last().unwrap()
}

// Doit prendre la possession
fn string_uppercase(mut data: &String) {
   data = data.to_uppercase();

   println!("{data}");
}

fn main() {
   let data = "Rust est génial!".to_string();

   get_char(&data);

   string_uppercase(data);
}
