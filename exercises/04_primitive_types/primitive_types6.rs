fn main() {
   // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
   #[test]
   fn indexing_tuple() {
       let numbers = (1, 2, 3);

       // TODO: Utilise un index de tuple pour accéder au deuxième élément de `numbers`
       // et assigne-le à une variable appelée `second`.
       // let second = ???;

       assert_eq!(second, 2, "Ce n'est pas le 2ème nombre du tuple !");
   }
}
