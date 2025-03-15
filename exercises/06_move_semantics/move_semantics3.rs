// TODO: Corrige l'erreur de compilation dans la fonction sans ajouter de nouvelle ligne.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
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
   fn move_semantics3() {
       let vec0 = vec![22, 44, 66];
       let vec1 = fill_vec(vec0);
       assert_eq!(vec1, [22, 44, 66, 88]);
   }
}
