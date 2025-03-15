fn main() {
   // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
   #[test]
   fn slice_out_of_array() {
       let a = [1, 2, 3, 4, 5];

       // TODO: Récupère une slice appelée `nice_slice` à partir du tableau `a` pour que le test passe.
       // let nice_slice = ???

       assert_eq!([2, 3, 4], nice_slice);
   }
}
