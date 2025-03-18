// 3 solutions possibles sont présentées.

// Avec une boucle `for` et une variable mutable.
fn factorielle_for(num: u64) -> u64 {
    let mut resultat = 1;

    for x in 2..=num {
        resultat *= x;
    }

    resultat
}

// Équivalent à `factorielle_for` mais plus court, sans boucle `for`
// ni variables mutables.
fn factorielle_fold(num: u64) -> u64 {
    // Cas num==0 : L'itérateur 2..=0 est vide
    //              -> La valeur initiale de `fold` est retournée, donc 1.
    // Cas num==1 : L'itérateur 2..=1 est aussi vide
    //              -> La valeur initiale 1 est retournée.
    // Cas num==2 : L'itérateur 2..=2 contient un élément
    //              -> La valeur initiale 1 est multipliée par 2 et le résultat
    //                 est retourné.
    // Cas num==3 : L'itérateur 2..=3 contient 2 éléments
    //              -> 1 * 2 est calculé, puis le résultat 2 est multiplié par
    //                 le second élément 3, donc 6 est retourné.
    // Et ainsi de suite…
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// Équivalent à `factorielle_fold` mais avec une méthode intégrée
// suggérée par Clippy.
fn factorielle_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorielle_de_0() {
        assert_eq!(factorielle_for(0), 1);
        assert_eq!(factorielle_fold(0), 1);
        assert_eq!(factorielle_product(0), 1);
    }

    #[test]
    fn factorielle_de_1() {
        assert_eq!(factorielle_for(1), 1);
        assert_eq!(factorielle_fold(1), 1);
        assert_eq!(factorielle_product(1), 1);
    }
    #[test]
    fn factorielle_de_2() {
        assert_eq!(factorielle_for(2), 2);
        assert_eq!(factorielle_fold(2), 2);
        assert_eq!(factorielle_product(2), 2);
    }

    #[test]
    fn factorielle_de_4() {
        assert_eq!(factorielle_for(4), 24);
        assert_eq!(factorielle_fold(4), 24);
        assert_eq!(factorielle_product(4), 24);
    }
}
