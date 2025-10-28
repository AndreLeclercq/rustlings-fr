fn main() {
    // Tu peux faire des tests ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Syntaxe d'indexation (indexing) du tuple.
        let second = numbers.1;

        assert_eq!(second, 2, "Ce n'est pas le 2ème nombre du tuple !");
    }
}
