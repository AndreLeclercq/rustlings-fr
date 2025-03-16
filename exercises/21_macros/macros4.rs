// TODO: Corrige l'erreur du compilateur en ajoutant un ou deux caractères.
#[rustfmt::skip]
macro_rules! my_macro {
   () => {
       println!("Regarde ma macro !");
   }
   ($val:expr) => {
       println!("Regarde cette autre macro : {}", $val);
   }
}

fn main() {
   my_macro!();
   my_macro!(7777);
}
