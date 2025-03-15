fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Tableau (Array)

    // TODO : Crée un vecteur appelé `v` contenant exactement les mêmes éléments que le tableau `a`.
    // Utilise la macro de vecteur.
    // let v = ???;

    (a, v)
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
