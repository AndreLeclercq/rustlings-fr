fn main() {
   // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
   // TODO: Corrige les erreurs de compilation uniquement en réorganisant les lignes dans le test.
   // N'ajoute, ne change ou ne supprime aucune ligne.
   #[test]
   fn move_semantics4() {
       let mut x = Vec::new();
       let y = &mut x;
       let z = &mut x;
       y.push(42);
       z.push(13);
       assert_eq!(x, [42, 13]);
   }
}
