// Tu peux utiliser le mot-clé `use` pour importer des chemins de modules depuis n'importe où,
// et particulièrement depuis la bibliothèque standard dans ton scope.

// TODO: Importe `SystemTime` et `UNIX_EPOCH` depuis le module `std::time` dans
// ton scope. Bonus de style si tu peux le faire en une seule ligne !
// use ???;

fn main() {
   match SystemTime::now().duration_since(UNIX_EPOCH) {
       Ok(n) => println!("Le 1970-01-01 00:00:00 UTC était il y a {} secondes !", n.as_secs()),
       Err(_) => panic!("SystemTime avant UNIX EPOCH !"),
   }
}
