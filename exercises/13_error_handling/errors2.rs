// Disons que nous écrivons un jeu où tu peux acheter des objets avec des jetons. Tous les objets coûtent
// 5 jetons, et à chaque achat il y a des frais de traitement de 1
// jeton. Un joueur du jeu va saisir combien d'objets il veut acheter, et
// la fonction `total_cost` calculera le coût total des objets. Comme
// le joueur a saisi la quantité, nous la recevons sous forme de chaîne de caractères. Il pourrait avoir
// saisi n'importe quoi, pas seulement des nombres !
//
// Actuellement, cette fonction ne gère pas du tout le cas d'erreur. Ce que nous voulons
// faire est : Si nous appelons la fonction `total_cost` avec une chaîne qui n'est pas un
// nombre, cette fonction renverra une `ParseIntError`. Dans ce cas, nous voulons
// immédiatement renvoyer cette erreur depuis notre fonction et ne pas essayer de multiplier et
// d'additionner.
//
// Il y a au moins deux façons de l'implémenter qui sont toutes deux correctes. Mais l'une
// est beaucoup plus courte !

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: Gère le cas d'erreur comme décrit ci-dessus.
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
