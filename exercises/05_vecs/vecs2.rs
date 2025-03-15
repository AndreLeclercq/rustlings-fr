fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO : Multiplie chaque élément du slice `input` par 2 et ajoute-le (push)
        // au vecteur `output`.
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // Un exemple de collection d'un vecteur après mapping.
    // On mappe chaque élément du slice `input` à sa valeur plus 1.
    // Si l'entrée est `[1, 2, 3]`, la sortie est `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO : Ici, on veut aussi multiplier chaque élément du slice `input`
    // par 2, mais en utilisant le mapping d'itérateur au lieu d'ajouter 
    // manuellement dans un vecteur vide.
    // Voir l'exemple dans la fonction `vec_map_example` ci-dessus.
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
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
