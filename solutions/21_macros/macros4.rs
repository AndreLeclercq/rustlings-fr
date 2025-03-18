// Ajout des points-virgules pour séparer les branches du macro.
#[rustfmt::skip]
macro_rules! my_macro {
   () => {
       println!("Découvre ma macro !");
   };
   ($val:expr) => {
       println!("Regarde cet autre macro : {}", $val);
   };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
