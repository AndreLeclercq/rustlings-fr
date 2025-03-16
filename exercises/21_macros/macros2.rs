fn main() {
   my_macro!();
}

// TODO: Corrige l'erreur du compilateur en déplaçant la définition complète de cette macro.
macro_rules! my_macro {
   () => {
       println!("Regarde ma macro !");
   };
}
