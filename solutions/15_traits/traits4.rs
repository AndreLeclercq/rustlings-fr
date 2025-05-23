trait Licensed {
    fn licensing_info(&self) -> String {
        "Licence par défaut".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    //                              ^^^^^^^^^^^^^             ^^^^^^^^^^^^^
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
