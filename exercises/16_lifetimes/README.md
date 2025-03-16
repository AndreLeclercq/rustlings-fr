# Durées de vie (lifetimes)

Les durées de vie (lifetimes) indiquent au compilateur comment vérifier si les références sont valides assez longtemps dans une situation donnée. Par exemple, elles disent "assure-toi que le paramètre 'a' vive aussi longtemps que le paramètre 'b' pour que la valeur de retour soit valide".

Elles ne sont nécessaires que pour les emprunts (borrows), c'est-à-dire les références, car les paramètres copiés ou déplacés sont possédés dans leur portée (scope) et ne peuvent pas être référencés en dehors. Les durées de vie permettent de vérifier que le code appelant, par exemple des fonctions, utilise des arguments valides. Elles sont restrictives pour leurs appelants.

Si tu souhaites en apprendre plus sur les annotations de durées de vie (lifetime annotations), le projet [lifetimekata](https://tfpk.github.io/lifetimekata/) propose des exercices similaires à Rustlings, mais entièrement consacrés à l'apprentissage de l'écriture des annotations de durées de vie.

## Informations complémentaires

- [Durées de vie (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validation des références avec les durées de vie](https://jimskapt.github.io/rust-book-fr/ch10-03-lifetime-syntax.html)
