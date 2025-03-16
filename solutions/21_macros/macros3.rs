// Ajout de l'attribut `macro_use`.
#[macro_use]
mod macros {
   macro_rules! my_macro {
       () => {
           println!("Découvre ma macro !");
       };
   }
}

fn main() {
   my_macro!();
}
