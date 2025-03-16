// Cet exercice est une version modifiée de l'exercice `errors4`. Il utilise des
// concepts que nous verrons plus tard dans le cours, comme `Box` et le trait
// `From`. Ce n'est pas important de les comprendre en détail maintenant, mais
// tu peux lire en avance si ça te plaît. Pour l'instant, considère le type
// `Box<dyn ???>` comme un type "je veux quelque chose qui fait ???".
//
// En bref, cette utilisation particulière des boîtes (boxes) est pour quand tu
// veux posséder une valeur et que tu te soucies seulement qu'elle implémente
// un trait particulier. Pour ce faire, la `Box` est déclarée comme
// `Box<dyn Trait>` où `Trait` est le trait que le compilateur recherche sur
// toute valeur utilisée dans ce contexte. Pour cet exercice, ce contexte
// concerne les erreurs potentielles qui peuvent être renvoyées dans un `Result`.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Ceci est requis pour que `CreationError` puisse implémenter `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "le nombre est négatif",
            CreationError::Zero => "le nombre est zéro",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("résultat={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
