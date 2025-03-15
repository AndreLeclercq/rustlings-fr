fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ ajouté

    vec.push(88);

    vec
}

fn main() {
    // Tu peux expérimenter ici si tu le souhaites.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // `vec0` n'est plus accessible car il a été déplacé vers `fill_vec`.
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
