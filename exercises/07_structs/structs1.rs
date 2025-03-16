struct ColorRegularStruct {
    // TODO: Ajoute les champs attendus par le test `regular_structs`.
    // Quels types devraient avoir les champs ? Quelles sont les valeurs minimales et maximales pour les couleurs RGB ?
}

struct ColorTupleStruct(/* TODO: Ajoute les champs attendus par le test `tuple_structs` */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instancie une struct régulière.
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instancie une tuple struct.
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instancie une unit struct.
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
