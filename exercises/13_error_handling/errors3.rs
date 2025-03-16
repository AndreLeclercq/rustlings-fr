// Voici un programme qui essaie d'utiliser une version complète de la
// fonction `total_cost` de l'exercice précédent. Mais ça ne fonctionne pas !
// Pourquoi ? Que devrions-nous faire pour le corriger ?

use std::num::ParseIntError;

// Ne change pas cette fonction.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Corrige l'erreur du compilateur en changeant la signature et le corps de la
// fonction `main`.
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Ne change pas cette ligne.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("Tu ne peux pas te permettre autant !");
    } else {
        tokens -= cost;
        println!("Tu as maintenant {tokens} jetons.");
    }
}
