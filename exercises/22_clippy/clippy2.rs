fn main() {
   let mut res = 42;
   let option = Some(12);
   // TODO: Corrige le lint Clippy.
   for x in option {
       res += x;
   }

   println!("{res}");
}
