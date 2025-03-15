fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
   let mut vec = vec;

   vec.push(88);

   vec
}

fn main() {
   // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
   use super::*;

   // TODO: Rends les deux vecteurs `vec0` et `vec1` accessibles en même temps pour
   // corriger l'erreur de compilation dans le test.
   #[test]
   fn move_semantics2() {
       let vec0 = vec![22, 44, 66];

       let vec1 = fill_vec(vec0);

       assert_eq!(vec0, [22, 44, 66]);
       assert_eq!(vec1, [22, 44, 66, 88]);
   }
}
