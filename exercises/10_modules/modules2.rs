// Tu peux importer des chemins de modules dans les scopes et leur donner de nouveaux noms
// avec les mots-clés `use` et `as`.

mod delicious_snacks {
   // TODO: Ajoute les deux déclarations `use` suivantes après les avoir corrigées.
   // use self::fruits::PEAR as ???;
   // use self::veggies::CUCUMBER as ???;

   mod fruits {
       pub const PEAR: &str = "Pear";
       pub const APPLE: &str = "Apple";
   }

   mod veggies {
       pub const CUCUMBER: &str = "Cucumber";
       pub const CARROT: &str = "Carrot";
   }
}

fn main() {
   println!(
       "en-cas préférés : {} et {}",
       delicious_snacks::fruit,
       delicious_snacks::veggie,
   );
}
