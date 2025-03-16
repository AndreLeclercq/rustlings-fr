fn trim_me(input: &str) -> &str {
   // TODO: Supprime les espaces blancs des deux extrémités d'une chaîne.
}

fn compose_me(input: &str) -> String {
   // TODO: Ajoute " world!" à la chaîne ! Il y a plusieurs façons de faire cela.
}

fn replace_me(input: &str) -> String {
   // TODO: Remplace "cars" dans la chaîne par "balloons".
}

fn main() {
   // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn trim_a_string() {
       assert_eq!(trim_me("Hello!     "), "Hello!");
       assert_eq!(trim_me("  What's up!"), "What's up!");
       assert_eq!(trim_me("   Hola!  "), "Hola!");
   }

   #[test]
   fn compose_a_string() {
       assert_eq!(compose_me("Hello"), "Hello world!");
       assert_eq!(compose_me("Goodbye"), "Goodbye world!");
   }

   #[test]
   fn replace_a_string() {
       assert_eq!(
           replace_me("I think cars are cool"),
           "I think balloons are cool",
       );
       assert_eq!(
           replace_me("I love to look at cars"),
           "I love to look at balloons",
       );
   }
}
