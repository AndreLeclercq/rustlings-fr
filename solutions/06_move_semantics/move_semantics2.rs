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

   #[test]
   fn move_semantics2() {
       let vec0 = vec![22, 44, 66];

       // On clone `vec0` pour que ce soit le clone qui soit déplacé dans `fill_vec`, 
       // et non `vec0` lui-même.
       let vec1 = fill_vec(vec0.clone());

       assert_eq!(vec0, [22, 44, 66]);
       assert_eq!(vec1, [22, 44, 66, 88]);
   }
}
