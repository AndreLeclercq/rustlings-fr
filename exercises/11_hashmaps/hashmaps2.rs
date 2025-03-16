// Nous collectons différents fruits pour préparer un délicieux gâteau aux fruits. Pour cela,
// nous avons un panier, que nous allons représenter sous forme de hash map (table de hachage). La clé
// représente le nom de chaque fruit que nous collectons et la valeur représente combien
// de ce fruit particulier nous avons collecté. Trois types de fruits -
// Pomme (4), Mangue (2) et Litchi (5) sont déjà dans le panier. Tu
// dois ajouter des fruits au panier pour qu'il y ait au moins un de chaque type et
// plus de 11 au total - nous avons beaucoup de bouches à nourrir. Tu n'as pas le droit
// d'ajouter plus des fruits qui sont déjà dans le panier (Pomme,
// Mangue et Litchi).

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Fruit {
   Pomme,
   Banane,
   Mangue,
   Litchi,
   Ananas,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
   let fruit_kinds = [
       Fruit::Pomme,
       Fruit::Banane,
       Fruit::Mangue,
       Fruit::Litchi,
       Fruit::Ananas,
   ];

   for fruit in fruit_kinds {
       // TODO: Insère de nouveaux fruits s'ils ne sont pas déjà présents dans le
       // panier. Note que tu n'as pas le droit d'ajouter un type de fruit qui est
       // déjà présent !
   }
}

fn main() {
   // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
   use super::*;

   // Ne modifie pas cette fonction !
   fn get_fruit_basket() -> HashMap<Fruit, u32> {
       let content = [(Fruit::Pomme, 4), (Fruit::Mangue, 2), (Fruit::Litchi, 5)];
       HashMap::from_iter(content)
   }

   #[test]
   fn test_given_fruits_are_not_modified() {
       let mut basket = get_fruit_basket();
       fruit_basket(&mut basket);
       assert_eq!(*basket.get(&Fruit::Pomme).unwrap(), 4);
       assert_eq!(*basket.get(&Fruit::Mangue).unwrap(), 2);
       assert_eq!(*basket.get(&Fruit::Litchi).unwrap(), 5);
   }

   #[test]
   fn at_least_five_types_of_fruits() {
       let mut basket = get_fruit_basket();
       fruit_basket(&mut basket);
       let count_fruit_kinds = basket.len();
       assert!(count_fruit_kinds >= 5);
   }

   #[test]
   fn greater_than_eleven_fruits() {
       let mut basket = get_fruit_basket();
       fruit_basket(&mut basket);
       let count = basket.values().sum::<u32>();
       assert!(count > 11);
   }

   #[test]
   fn all_fruit_types_in_basket() {
       let fruit_kinds = [
           Fruit::Pomme,
           Fruit::Banane,
           Fruit::Mangue,
           Fruit::Litchi,
           Fruit::Ananas,
       ];

       let mut basket = get_fruit_basket();
       fruit_basket(&mut basket);

       for fruit_kind in fruit_kinds {
           let Some(amount) = basket.get(&fruit_kind) else {
               panic!("Le type de fruit {fruit_kind:?} n'a pas été trouvé dans le panier");
           };
           assert!(*amount > 0);
       }
   }
}
