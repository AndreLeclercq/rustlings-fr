// Ce wrapper puissant permet de stocker une valeur de type entier positif.
// TODO: Réécris-le en utilisant un générique (generic) pour qu'il supporte l'ENROBAGE (wrapping) DE N'IMPORTE QUEL type.
struct Wrapper {
    value: u32,
}

// TODO: Adapte l'implémentation de la structure pour qu'elle soit générique sur la valeur enrobée.
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
