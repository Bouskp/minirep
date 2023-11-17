pub struct Config {
    pub recherche : String,
    pub fichier : String,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3  {
            return Err("il n'y a pas assez de paramètres.");
        }
        let recherche = args[1];
        let fichier = args[2];

        Config {
            recherche,
            fichier,
        }
    }

    pub fn get_recherche(config : &Config) -> String {
        config.recherche
    }

    pub fn get_fichier(config : &Config) -> String {
        config.fichier
    }
}


// Ouverture de fichier

// Lecture du fichier


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
