fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    // TODO: Corrige les erreurs du compilateur uniquement en réorganisant les lignes dans le test.
    // N'ajoute, ne change ou ne supprime aucune ligne.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // `y` est utilisé ici.
        y.push(42);
        // La référence mutable `y` n'est plus utilisée,
        // donc une nouvelle référence peut être créée.
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
