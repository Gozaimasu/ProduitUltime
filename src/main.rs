use std::io;

fn main() {
    let mut nb_articles = String::new();

    io::stdin()
        .read_line(&mut nb_articles)
        .expect("Échec de la lecture de l'entrée utilisateur");

    let nb_articles: u32 = nb_articles
        .trim()
        .parse()
        .expect("Il ne s'agit pas d'un u32");

    if nb_articles == 0 || nb_articles > 40 {
        panic!("{} en dehors des bornes", nb_articles);
    }

    let mut max_score = 0;
    let mut article_gagnant = String::new();

    for _ in 0..nb_articles {
        let mut ligne_score = String::new();

        io::stdin()
            .read_line(&mut ligne_score)
            .expect("Échec de la lecture de l'entrée utilisateur");

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
