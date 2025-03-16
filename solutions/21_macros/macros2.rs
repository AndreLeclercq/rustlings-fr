// Le macro est maintenant défini avant son appel.
macro_rules! my_macro {
   () => {
       println!("Découvre ma macro !");
   };
}

fn main() {
   my_macro!();
}
