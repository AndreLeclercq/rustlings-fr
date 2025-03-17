// `TryFrom` est une conversion de type simple et sûre qui peut échouer de manière contrôlée
// dans certaines circonstances. Fondamentalement, c'est la même chose que `From`. La principale
// différence est que cela devrait renvoyer un type `Result` au lieu du type
// cible lui-même. Tu peux en savoir plus dans la documentation :
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html

#![allow(clippy::useless_vec)]
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
   red: u8,
   green: u8,
   blue: u8,
}

// Nous utiliserons ce type d'erreur pour les conversions `TryFrom`.
#[derive(Debug, PartialEq)]
enum IntoColorError {
   // Longueur incorrecte de la tranche
   BadLen,
   // Erreur de conversion d'entier
   IntConversion,
}

// TODO: Implémentation du tuple.
// Les valeurs correctes de couleur RGB doivent être des entiers dans la plage 0..=255.
impl TryFrom<(i16, i16, i16)> for Color {
   type Error = IntoColorError;

   fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {}
}

// TODO: Implémentation du tableau.
impl TryFrom<[i16; 3]> for Color {
   type Error = IntoColorError;

   fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {}
}

// TODO: Implémentation de la tranche (slice).
// Cette implémentation doit vérifier la longueur de la tranche.
impl TryFrom<&[i16]> for Color {
   type Error = IntoColorError;

   fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {}
}

fn main() {
   // Utilisation de la fonction `try_from`.
   let c1 = Color::try_from((183, 65, 14));
   println!("{c1:?}");

   // Puisque `TryFrom` est implémenté pour `Color`, nous pouvons utiliser `TryInto`.
   let c2: Result<Color, _> = [183, 65, 14].try_into();
   println!("{c2:?}");

   let v = vec![183, 65, 14];
   // Avec une tranche, nous devrions utiliser la fonction `try_from`
   let c3 = Color::try_from(&v[..]);
   println!("{c3:?}");
   // ou mettre la tranche entre parenthèses et utiliser `try_into`.
   let c4: Result<Color, _> = (&v[..]).try_into();
   println!("{c4:?}");
}

#[cfg(test)]
mod tests {
   use super::*;
   use IntoColorError::*;

   #[test]
   fn test_tuple_out_of_range_positive() {
       assert_eq!(Color::try_from((256, 1000, 10000)), Err(IntConversion));
   }

   #[test]
   fn test_tuple_out_of_range_negative() {
       assert_eq!(Color::try_from((-1, -10, -256)), Err(IntConversion));
   }

   #[test]
   fn test_tuple_sum() {
       assert_eq!(Color::try_from((-1, 255, 255)), Err(IntConversion));
   }

   #[test]
   fn test_tuple_correct() {
       let c: Result<Color, _> = (183, 65, 14).try_into();
       assert!(c.is_ok());
       assert_eq!(
           c.unwrap(),
           Color {
               red: 183,
               green: 65,
               blue: 14,
           }
       );
   }

   #[test]
   fn test_array_out_of_range_positive() {
       let c: Result<Color, _> = [1000, 10000, 256].try_into();
       assert_eq!(c, Err(IntConversion));
   }

   #[test]
   fn test_array_out_of_range_negative() {
       let c: Result<Color, _> = [-10, -256, -1].try_into();
       assert_eq!(c, Err(IntConversion));
   }

   #[test]
   fn test_array_sum() {
       let c: Result<Color, _> = [-1, 255, 255].try_into();
       assert_eq!(c, Err(IntConversion));
   }

   #[test]
   fn test_array_correct() {
       let c: Result<Color, _> = [183, 65, 14].try_into();
       assert!(c.is_ok());
       assert_eq!(
           c.unwrap(),
           Color {
               red: 183,
               green: 65,
               blue: 14
           }
       );
   }

   #[test]
   fn test_slice_out_of_range_positive() {
       let arr = [10000, 256, 1000];
       assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
   }

   #[test]
   fn test_slice_out_of_range_negative() {
       let arr = [-256, -1, -10];
       assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
   }

   #[test]
   fn test_slice_sum() {
       let arr = [-1, 255, 255];
       assert_eq!(Color::try_from(&arr[..]), Err(IntConversion));
   }

   #[test]
   fn test_slice_correct() {
       let v = vec![183, 65, 14];
       let c: Result<Color, _> = Color::try_from(&v[..]);
       assert!(c.is_ok());
       assert_eq!(
           c.unwrap(),
           Color {
               red: 183,
               green: 65,
               blue: 14,
           }
       );
   }

   #[test]
   fn test_slice_excess_length() {
       let v = vec![0, 0, 0, 0];
       assert_eq!(Color::try_from(&v[..]), Err(BadLen));
   }

   #[test]
   fn test_slice_insufficient_length() {
       let v = vec![0, 0];
       assert_eq!(Color::try_from(&v[..]), Err(BadLen));
   }
}
