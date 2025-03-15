fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- indices
        //          -------
        //             |
        //             +--- slice (tranche)

        // Remarque que l'index supérieur 4 est exclu.
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // L'index supérieur peut être inclus en utilisant la syntaxe `..=` (avec le signe `=`)
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
