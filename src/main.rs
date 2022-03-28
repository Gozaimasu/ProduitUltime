use produit_ultime::{get_string_from_input, get_u32_from_input};
use std::io;

use crate::toto::ArticleGagnant;
pub mod toto;

fn main() {
    // Récupération du nombre d'article à partir de stdin
    let nb_articles = get_u32_from_input(&mut io::stdin().lock());

    // On panique si pas le bon nombre
    if nb_articles == 0 || nb_articles > 40 {
        panic!("{} en dehors des bornes", nb_articles);
    }
    // Création de l'article gagnant
    let mut article_gagnant = ArticleGagnant::new();

    // On parcourt les articles ...
    for _ in 0..nb_articles {
        // ... en les lisant à partir de stdin
        let ligne_score = get_string_from_input(&mut io::stdin().lock());

        // On met à jour l'article gagnant
        article_gagnant.update_from_input(&ligne_score);
    }

    println!("{}", article_gagnant.libelle.as_ref().unwrap());
}
