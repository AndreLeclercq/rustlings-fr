// Imaginons un jeu où tu achètes des articles avec des jetons. Chaque article coûte
// 5 jetons, et à chaque achat il y a des frais de traitement de 1 jeton. Le joueur
// va saisir le nombre d'articles qu'il souhaite acheter, et la fonction `total_cost`
// calculera le coût total des articles. Comme le joueur saisit la quantité sous forme
// de chaîne de caractères, il peut avoir saisi n'importe quoi, pas forcément des nombres !
//
// Actuellement, cette fonction ne gère pas du tout le cas d'erreur. Ce qu'on veut,
// c'est : si on appelle la fonction `total_cost` avec une chaîne qui n'est pas un
// nombre, elle renverra un `ParseIntError`. Dans ce cas, on veut immédiatement
// renvoyer cette erreur sans essayer de multiplier et d'additionner.
//
// Il existe au moins deux façons correctes d'implémenter ça. Mais l'une est
// beaucoup plus courte !

use std::num::ParseIntError;

#[allow(unused_variables, clippy::question_mark)]
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // Ajout de `?` pour propager l'erreur.
    let qty = item_quantity.parse::<i32>()?;
    //                                    ^ ajouté

    // Équivalent à cette version verbeuse :
    let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn quantite_article_est_un_nombre_valide() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn quantite_article_est_un_nombre_invalide() {
        assert_eq!(
            total_cost("bip boup").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
