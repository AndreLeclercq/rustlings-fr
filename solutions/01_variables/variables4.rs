fn main() {
   // En Rust, les variables sont immuables par défaut.
   // Ajouter le mot-clé `mut` après `let` rend la variable déclarée mutable.
   let mut x = 3;
   println!("Nombre {x}");

   x = 5;
   println!("Nombre {x}");
}
