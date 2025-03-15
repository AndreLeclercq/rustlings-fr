fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Un exemple de collecte d'un vecteur après mappage.
    // Nous mappons chaque élément de la slice `input` à sa valeur plus 1.
    // Si l'entrée est `[1, 2, 3]`, la sortie est `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // Nous approfondirons les iterators (itérateurs), mais pour l'instant, c'est tout ce que tu
    // avais à faire !
    // Note avancée : Cette méthode est plus efficace car elle préalloue automatiquement
    // assez de capacité. Cela peut être fait manuellement dans `vec_loop`
    // en utilisant `Vec::with_capacity(input.len())` au lieu de `Vec::new()`.
    input.iter().map(|element| 2 * element).collect()
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
