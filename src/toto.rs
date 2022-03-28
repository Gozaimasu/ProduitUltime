pub struct ArticleGagnant {
    score: Option<u32>,
    pub libelle: Option<String>,
}

impl ArticleGagnant {
    pub fn new() -> Self {
        ArticleGagnant { score: None, libelle: None }
    }

    pub fn update_from_input(&mut self, input: &str) {
        let mut split = input.split(' ').into_iter();

        let score: u32 = split
            .next()
            .unwrap()
            .parse()
            .expect("Il ne s'agit pas d'un u32");

        if let Some(current_score) = self.score {
            // Il y a déjà une valeur
            if current_score < score {
                self.score = Some(score);
                self.libelle = Some(split.next().unwrap().to_string());
            }
        } else {
            // None actuellement
            self.score = Some(score);
            self.libelle = Some(split.next().unwrap().to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_from_input() {
        let mut article_gagnant = ArticleGagnant::new();

        article_gagnant.update_from_input("1 input");

        assert_eq!("input", article_gagnant.libelle.as_ref().unwrap());

        article_gagnant.update_from_input("3 input2");

        assert_eq!("input2", article_gagnant.libelle.as_ref().unwrap());

        article_gagnant.update_from_input("2 input3");

        assert_eq!("input2", article_gagnant.libelle.as_ref().unwrap());
    }
}