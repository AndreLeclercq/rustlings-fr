// C'est similaire à l'exercice `from_into` précédent. Mais cette fois, nous allons
// implémenter `FromStr` et renvoyer des erreurs au lieu de revenir à une valeur par défaut.
// De plus, après avoir implémenté `FromStr`, tu peux utiliser la méthode `parse`
// sur les chaînes pour générer un objet du type implémenteur. Tu peux en savoir plus
// dans la documentation :
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
    // Champ de nom vide
    NoName,
    // Erreur encapsulée de parse::<u8>()
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let (Some(name), Some(age), None) = (split.next(), split.next(), split.next()) else {
            //                      ^^^^ il ne devrait pas y avoir de troisième élément
            return Err(ParsePersonError::BadLen);
        };

        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age = age.parse().map_err(ParsePersonError::ParseInt)?;

        Ok(Self {
            name: name.into(),
            age,
        })
    }
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
