mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // Ajout de `pub` avant `fn` pour rendre la fonction accessible en dehors du module.
    pub fn make_sausage() {
        get_secret_recipe();
        println!("saucisse !");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
