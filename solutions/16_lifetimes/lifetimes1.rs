// Le compilateur Rust a besoin de savoir comment vérifier si les références fournies sont
// valides, afin de pouvoir informer le programmeur si une référence risque de
// sortir de son scope (portée) avant d'être utilisée. Rappelle-toi, les références sont
// des emprunts (borrows) et ne possèdent pas leurs propres données. Que se passe-t-il
// si leur propriétaire (owner) sort de son scope ?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //    ^^^^     ^^          ^^          ^^
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
