// TODO: Répare l'erreur de compilation dans cette fonction.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
   let vec = vec;

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
   fn move_semantics1() {
       let vec0 = vec![22, 44, 66];
       let vec1 = fill_vec(vec0);
       assert_eq!(vec1, vec![22, 44, 66, 88]);
   }
}
