// C'est similaire à l'exercice précédent `from_into`. Mais cette fois, nous
// allons implémenter `FromStr` et renvoyer des erreurs au lieu de recourir à une valeur
// par défaut. De plus, après avoir implémenté `FromStr`, tu peux utiliser la méthode
// `parse` sur les chaînes pour générer un objet du type implémenteur. Tu peux en
// savoir plus dans la documentation :
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
   name: String,
   age: u8,
}

// Nous utiliserons ce type d'erreur pour l'implémentation de `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
   // Nombre incorrect de champs
   BadLen,
   // Champ nom vide
   NoName,
   // Erreur encapsulée de parse::<u8>()
   ParseInt(ParseIntError),
}

// TODO: Complète cette implémentation de `FromStr` pour pouvoir analyser une `Person`
// à partir d'une chaîne sous la forme "Mark,20".
// Note que tu devras analyser la composante âge en `u8` avec quelque chose
// comme `"4".parse::<u8>()`.
//
// Étapes :
// 1. Divise la chaîne donnée sur les virgules présentes.
// 2. Si l'opération de division renvoie moins ou plus de 2 éléments, renvoie
//    l'erreur `ParsePersonError::BadLen`.
// 3. Utilise le premier élément de l'opération de division comme nom.
// 4. Si le nom est vide, renvoie l'erreur `ParsePersonError::NoName`.
// 5. Analyse le deuxième élément de l'opération de division en `u8` comme âge.
// 6. Si l'analyse de l'âge échoue, renvoie l'erreur `ParsePersonError::ParseInt`.
impl FromStr for Person {
   type Err = ParsePersonError;

   fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
   let p = "Mark,20".parse::<Person>();
   println!("{p:?}");
}

#[cfg(test)]
mod tests {
   use super::*;
   use ParsePersonError::*;

   #[test]
   fn empty_input() {
       assert_eq!("".parse::<Person>(), Err(BadLen));
   }

   #[test]
   fn good_input() {
       let p = "John,32".parse::<Person>();
       assert!(p.is_ok());
       let p = p.unwrap();
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 32);
   }

   #[test]
   fn missing_age() {
       assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
   }

   #[test]
   fn invalid_age() {
       assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
   }

   #[test]
   fn missing_comma_and_age() {
       assert_eq!("John".parse::<Person>(), Err(BadLen));
   }

   #[test]
   fn missing_name() {
       assert_eq!(",1".parse::<Person>(), Err(NoName));
   }

   #[test]
   fn missing_name_and_age() {
       assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
   }

   #[test]
   fn missing_name_and_invalid_age() {
       assert!(matches!(
           ",one".parse::<Person>(),
           Err(NoName | ParseInt(_)),
       ));
   }

   #[test]
   fn trailing_comma() {
       assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
   }

   #[test]
   fn trailing_comma_and_some_string() {
       assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
   }
}
