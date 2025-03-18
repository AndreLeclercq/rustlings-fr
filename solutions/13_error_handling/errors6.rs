// Utiliser des types d'erreurs génériques comme `Box<dyn Error>` n'est pas recommandé
// pour du code de bibliothèque où les appelants pourraient vouloir prendre des
// décisions basées sur le contenu de l'erreur plutôt que de l'imprimer ou de la
// propager. Ici, nous définissons un type d'erreur personnalisé pour permettre
// aux appelants de décider quoi faire quand notre fonction renvoie une erreur.

use std::num::ParseIntError;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Un type d'erreur personnalisé que nous utiliserons dans
// `PositiveNonzeroInteger::parse`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> Self {
        Self::Creation(err)
    }

    fn from_parse_int(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

// Comme solution alternative, implémenter le trait `From` permet la conversion
// automatique d'un `ParseIntError` en `ParsePosNonzeroError` en utilisant
// l'opérateur `?`, sans avoir besoin d'appeler `map_err`.
//
// ```
// let x: i64 = s.parse()?;
// ```
//
// Les traits comme `From` seront traités dans des exercices ultérieurs.
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(err)
    }
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(Self(x as u64)),
        }
    }

    fn parse(s: &str) -> Result<Self, ParsePosNonzeroError> {
        // Renvoie une erreur appropriée au lieu de paniquer quand `parse()`
        // retourne une erreur.
        let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse_int)?;
        //                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        Self::new(x).map_err(ParsePosNonzeroError::from_creation)
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            PositiveNonzeroInteger::parse("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_)),
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::parse("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative)),
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::parse("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero)),
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42).unwrap();
        assert_eq!(x.0, 42);
        assert_eq!(PositiveNonzeroInteger::parse("42"), Ok(x));
    }
}
