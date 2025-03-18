#![allow(dead_code)]

mod delicious_snacks {
    // Ajout de `pub` et utilisation de l'alias attendu après `as`.
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

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
