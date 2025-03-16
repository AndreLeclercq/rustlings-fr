// Une liste de scores (un par ligne) d'un match de football est donnée. Chaque ligne est de
// la forme "<nom_équipe_1>,<nom_équipe_2>,<buts_équipe_1>,<buts_équipe_2>"
// Exemple: "Angleterre,France,4,2" (l'Angleterre a marqué 4 buts, la France 2).
//
// Tu dois construire un tableau de scores contenant le nom de l'équipe, le nombre total
// de buts marqués par l'équipe, et le nombre total de buts encaissés par l'équipe.

use std::collections::HashMap;

// Une structure pour stocker les détails des buts d'une équipe.
#[derive(Default)]
struct TeamScores {
   goals_scored: u8,
   goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
   // Le nom de l'équipe est la clé et sa structure associée est la valeur.
   let mut scores = HashMap::<&str, TeamScores>::new();

   for line in results.lines() {
       let mut split_iterator = line.split(',');
       // NOTE: Nous utilisons `unwrap` car nous n'avons pas encore traité la gestion d'erreurs.
       let team_1_name = split_iterator.next().unwrap();
       let team_2_name = split_iterator.next().unwrap();
       let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
       let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

       // TODO: Remplis le tableau de scores avec les détails extraits.
       // Garde à l'esprit que les buts marqués par l'équipe 1 seront le nombre de buts
       // encaissés par l'équipe 2. De même, les buts marqués par l'équipe 2 seront le
       // nombre de buts encaissés par l'équipe 1.
   }

   scores
}

fn main() {
   // Tu peux expérimenter ici si tu veux.
}

#[cfg(test)]
mod tests {
   use super::*;

   const RESULTS: &str = "Angleterre,France,4,2
France,Italie,3,1
Pologne,Espagne,2,0
Allemagne,Angleterre,2,1
Angleterre,Espagne,1,0";

   #[test]
   fn build_scores() {
       let scores = build_scores_table(RESULTS);

       assert!(["Angleterre", "France", "Allemagne", "Italie", "Pologne", "Espagne"]
           .into_iter()
           .all(|team_name| scores.contains_key(team_name)));
   }

   #[test]
   fn validate_team_score_1() {
       let scores = build_scores_table(RESULTS);
       let team = scores.get("Angleterre").unwrap();
       assert_eq!(team.goals_scored, 6);
       assert_eq!(team.goals_conceded, 4);
   }

   #[test]
   fn validate_team_score_2() {
       let scores = build_scores_table(RESULTS);
       let team = scores.get("Espagne").unwrap();
       assert_eq!(team.goals_scored, 0);
       assert_eq!(team.goals_conceded, 3);
   }
}
