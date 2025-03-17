// Le trait `From` est utilisé pour les conversions de valeur à valeur. Si `From` est
// implémenté, une implémentation de `Into` est automatiquement fournie.
// Tu peux en savoir plus dans la documentation :
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
   name: String,
   age: u8,
}

// Nous implémentons le trait Default pour l'utiliser comme solution de repli quand la chaîne
// fournie n'est pas convertible en un objet `Person`.
impl Default for Person {
   fn default() -> Self {
       Self {
           name: String::from("John"),
           age: 30,
       }
   }
}

impl From<&str> for Person {
   fn from(s: &str) -> Self {
       let mut split = s.split(',');
       let (Some(name), Some(age), None) = (split.next(), split.next(), split.next()) else {
           //                      ^^^^ il ne devrait pas y avoir de troisième élément
           return Self::default();
       };

       if name.is_empty() {
           return Self::default();
       }

       let Ok(age) = age.parse() else {
           return Self::default();
       };

       Self {
           name: name.into(),
           age,
       }
   }
}

fn main() {
   // Utilise la fonction `from`.
   let p1 = Person::from("Mark,20");
   println!("{p1:?}");

   // Puisque `From` est implémenté pour Person, nous pouvons utiliser `Into`.
   let p2: Person = "Gerald,70".into();
   println!("{p2:?}");
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_default() {
       let dp = Person::default();
       assert_eq!(dp.name, "John");
       assert_eq!(dp.age, 30);
   }

   #[test]
   fn test_bad_convert() {
       let p = Person::from("");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_good_convert() {
       let p = Person::from("Mark,20");
       assert_eq!(p.name, "Mark");
       assert_eq!(p.age, 20);
   }

   #[test]
   fn test_bad_age() {
       let p = Person::from("Mark,twenty");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_missing_comma_and_age() {
       let p: Person = Person::from("Mark");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_missing_age() {
       let p: Person = Person::from("Mark,");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_missing_name() {
       let p: Person = Person::from(",1");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_missing_name_and_age() {
       let p: Person = Person::from(",");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_missing_name_and_invalid_age() {
       let p: Person = Person::from(",one");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_trailing_comma() {
       let p: Person = Person::from("Mike,32,");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }

   #[test]
   fn test_trailing_comma_and_some_string() {
       let p: Person = Person::from("Mike,32,dog");
       assert_eq!(p.name, "John");
       assert_eq!(p.age, 30);
   }
}
