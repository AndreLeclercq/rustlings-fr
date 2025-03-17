// Voici quelques corrections Clippy faciles pour que tu puisses voir son utilité 📎
// TODO: Corrige tous les lints Clippy.

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
   let my_option: Option<()> = None;
   if my_option.is_none() {
       println!("{:?}", my_option.unwrap());
   }

   let my_arr = &[
       -1, -2, -3
       -4, -5, -6
   ];
   println!("Mon tableau ! Le voici : {my_arr:?}");

   let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
   println!("Ce Vec est vide, tu vois ? {my_empty_vec:?}");

   let mut value_a = 45;
   let mut value_b = 66;
   // Échangeons ces deux valeurs !
   value_a = value_b;
   value_b = value_a;
   println!("valeur a : {value_a}; valeur b : {value_b}");
}
