# Conversions de types

Rust offre de nombreuses façons de convertir une valeur d'un type donné en un autre type.

La forme la plus simple de conversion de type est l'expression de cast de type. Elle est représentée par l'opérateur binaire `as`. Par exemple, `println!("{}", 1 + 1.0);` ne compilera pas, car `1` est un entier tandis que `1.0` est un nombre à virgule flottante. Cependant, `println!("{}", 1 as f32 + 1.0)` devrait compiler. L'exercice [`using_as`](using_as.rs) essaie de couvrir ce sujet.

Rust propose également des traits qui facilitent les conversions de types lors de leur implémentation. Ces traits se trouvent dans le module [`convert`](https://doc.rust-lang.org/std/convert/index.html).
Les traits sont les suivants :

- `From` et `Into` couverts dans [`from_into`](from_into.rs)
- `TryFrom` et `TryInto` couverts dans [`try_from_into`](try_from_into.rs)
- `AsRef` et `AsMut` couverts dans [`as_ref_mut`](as_ref_mut.rs)

De plus, le module `std::str` offre un trait appelé [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) qui aide à convertir des chaînes en types cibles via la méthode `parse` sur les chaînes. Si correctement implémenté pour un type donné `Person`, alors `let p: Person = "Mark,20".parse().unwrap()` devrait à la fois compiler et s'exécuter sans paniquer.

Ce sont les principales façons ***dans la bibliothèque standard*** de convertir des données vers les types souhaités.

## Informations complémentaires

Ces sujets ne sont pas directement couverts dans le rust book, mais la bibliothèque standard offre une excellente documentation à ce sujet.

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [trait `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)
