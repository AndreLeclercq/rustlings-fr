// Cet exercice est une version modifiée de l'exercice `errors4`. Il utilise certains
// concepts que nous ne verrons que plus tard dans le cours, comme `Box` et le
// trait `From`. Il n'est pas important de les comprendre en détail maintenant, mais
// tu peux lire en avance si tu le souhaites. Pour l'instant, considère le type `Box<dyn ???>` comme
// un type "Je veux n'importe quoi qui fait ???".
//
// En bref, ce cas d'utilisation particulier des box est pour quand tu veux posséder une
// valeur et que tu te préoccupes uniquement qu'elle soit d'un type qui implémente un trait
// particulier. Pour ce faire, la `Box` est déclarée comme étant de type `Box<dyn Trait>` où
// `Trait` est le trait que le compilateur recherche sur toute valeur utilisée dans ce
// contexte. Pour cet exercice, ce contexte est celui des erreurs potentielles qui
// peuvent être renvoyées dans un `Result`.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Ceci est nécessaire pour que `CreationError` puisse implémenter `Error`.
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

// TODO: Ajoute le type de retour correct `Result<(), Box<dyn ???>>`. Que pouvons-nous
// utiliser pour décrire les deux erreurs ? Y a-t-il un trait que les deux erreurs implémentent ?
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
