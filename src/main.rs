use std::io;
use produit_ultime::{get_u32_from_input, get_string_from_input};

fn main() {
    let nb_articles = get_u32_from_input(&mut io::stdin());

    if nb_articles == 0 || nb_articles > 40 {
        panic!("{} en dehors des bornes", nb_articles);
    }

    let mut max_score = 0;
    let mut article_gagnant = String::new();

    for _ in 0..nb_articles {
        let ligne_score = get_string_from_input(&mut io::stdin());

        let mut split = ligne_score.split(' ').into_iter();

        let score: u32 = split
            .next()
            .unwrap()
            .parse()
            .expect("Il ne s'agit pas d'un u32");
        if score > max_score {
            max_score = score;
            article_gagnant = split.next().unwrap().to_string();
        }
    }

    println!("{}", article_gagnant);
}
