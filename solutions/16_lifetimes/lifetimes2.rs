fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("une longue chaîne est longue");
    // Solution 1 : Tu peux déplacer `string2` hors du bloc interne pour qu'elle ne
    // soit pas libérée (dropped) avant l'instruction d'impression.
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("La chaîne la plus longue est '{result}'");
    // `string2` libérée à la fin de la fonction.

    // =========================================================================

    let string1 = String::from("une longue chaîne est longue");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // Solution 2 : Tu peux déplacer l'instruction d'impression dans le bloc interne
        // pour qu'elle soit exécutée avant que `string2` ne soit libérée.
        println!("La chaîne la plus longue est '{result}'");
        // `string2` libérée ici (fin du scope interne).
    }
}
