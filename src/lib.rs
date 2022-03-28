use std::io::BufRead;

pub fn get_u32_from_input(input: &mut impl BufRead) -> u32 {
    let mut parameter = String::new();

    input
        .read_line(&mut parameter)
        .expect("Échec de la lecture de l'entrée utilisateur");

    parameter.trim().parse().expect("Il ne s'agit pas d'un u32")
}

pub fn get_string_from_input(input: &mut impl BufRead) -> String {
    let mut parameter = String::new();

    input
        .read_line(&mut parameter)
        .expect("Échec de la lecture de l'entrée utilisateur");

    parameter.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_u32_from_u32_input() {
        let result = get_u32_from_input(&mut "32\n".as_bytes());

        assert_eq!(32, result);
    }

    #[test]
    #[should_panic]
    fn get_u32_from_string_input() {
        get_u32_from_input(&mut "string\n".as_bytes());
    }

    #[test]
    fn get_string_from_string_input() {
        let result = get_string_from_input(&mut "32\n".as_bytes());

        assert_eq!("32", result);
    }
}
