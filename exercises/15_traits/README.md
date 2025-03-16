# Traits (caractéristiques)

Un trait est un ensemble de méthodes.

Les types de données peuvent implémenter des traits. Pour ce faire, les méthodes qui composent le trait sont définies pour le type de données. Par exemple, le type de données `String` implémente le trait `From<&str>`. Cela permet à un utilisateur d'écrire `String::from("hello")`.

De cette manière, les traits ressemblent un peu aux interfaces Java et aux classes abstraites C++.

Quelques traits Rust courants incluent :

- `Clone` (la méthode `clone`)
- `Display` (qui permet un affichage formaté via `{}`)
- `Debug` (qui permet un affichage formaté via `{:?}`)

Comme les traits indiquent un comportement partagé entre les types de données, ils sont utiles lors de l'écriture de génériques (generics).

## Informations complémentaires

- [Traits](https://jimskapt.github.io/rust-book-fr/ch10-02-traits.html)
