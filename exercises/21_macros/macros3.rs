// TODO: Corrige l'erreur du compilateur sans sortir la définition de la macro de ce
// module.
mod macros {
   macro_rules! my_macro {
       () => {
           println!("Regarde ma macro !");
       };
   }
}

fn main() {
   my_macro!();
}
