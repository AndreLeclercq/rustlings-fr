// Définissons un modèle simple pour suivre la progression des exercices Rustlings. 
// La progression sera modélisée à l'aide d'une table de hachage (hash map). 
// Le nom de l'exercice est la clé et la progression est la valeur. 
// Deux fonctions de comptage ont été créées pour compter le nombre 
// d'exercices avec une progression donnée. Recrée cette fonctionnalité de 
// comptage en utilisant des itérateurs. Essaie de ne pas utiliser de boucles impératives (for/while).

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if *val == value {
            count += 1;
        }
    }
    count
}

// TODO : Implémente la fonctionnalité de `count_for` mais avec un itérateur 
// au lieu d'une boucle `for`.
fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // `map` est une table de hachage avec des clés `String` et des valeurs `Progress`.
    // map = { "variables1": Complete, "from_str": None, … }
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

// TODO : Implémente la fonctionnalité de `count_collection_for` mais avec 
// un itérateur au lieu d'une boucle `for`.
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // `collection` est un tableau de tables de hachage.
    // collection = [{ "variables1": Complete, "from_str": None, … },
    //               { "variables2": Complete, … }, … ]
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Complete), 3);
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::Some), 1);
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(count_iterator(&map, Progress::None), 2);
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(
                count_for(&map, progress_state),
                count_iterator(&map, progress_state),
            );
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_iterator(&collection, Progress::Complete),
            6,
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::Some), 1);
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(count_collection_iterator(&collection, Progress::None), 4);
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        let progress_states = [Progress::Complete, Progress::Some, Progress::None];

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state),
            );
        }
    }
}
