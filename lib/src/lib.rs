use std::collections::HashMap;
use std::fs::File;
use std::{env, io};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    pub recherche : String,
    pub fichier : String,
    pub is_sensitive : bool,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3  {
            return Err("il n'y a pas assez de paramètres.");
        }
        let recherche = &args[1];
        let fichier = &args[2];
        let is_sensible = env::var("SENSIBLE_CASE").is_err();

        Ok(Config {
            recherche : recherche.to_string(),
            fichier : fichier.to_string(),
            is_sensitive:is_sensible
        })
    }

    pub fn get_recherche(&self) -> String {
        self.recherche.to_string()
    }

    pub fn get_fichier(&self) -> String {
        self.fichier.to_string()
    }

    pub fn set_recherche(&mut self, value : &String) {
        self.recherche = value.to_string();
    }

    pub fn set_fichier(&mut self, value: &String) {
        self.fichier = value.to_string();
    }
}


// Ouverture de fichier
pub fn ouverture(config : &Config)-> Result<File, io::Error> {
    match File::open(config.get_fichier()) {
        Err(e) => Err(e),
        Ok(f) => Ok(f),
    }
}

// Lecture du fichier
pub fn lecture(file: &mut File) -> BufReader<&mut File> {
     let reader = BufReader::new(file);
     return reader;
}

pub fn recherche(config: &Config, contenu : BufReader<&mut File>) -> HashMap<usize, String> {
    let mut result = HashMap::new();

    for (line, value) in contenu.lines().enumerate() {
        let value = value.ok().unwrap();
        if value.contains(&config.recherche) {
            result.insert(line, value);
        }
    }

    return result;
}

pub fn presentation(resultat: HashMap<usize, String>, config: &Config) {
    if resultat.len() == 0  {
        println!("il n'y a pas le mot: {} dans le fichier: {}", config.recherche, config.fichier);
    }
    for line in resultat {
        println!("A la ligne {} : {}", line.0 + 1, line.1);
    }
}

pub fn sensitive_fun(config: &Config, contenu : BufReader<&mut File>) -> HashMap<usize, String>{
    let mut result = HashMap::new();

    let recherche = config.recherche.to_lowercase();

    for (line, value) in contenu.lines().enumerate() {
        let value_clone= value.ok().unwrap().clone();
        let value_lower = value_clone.to_lowercase();
        if value_lower.contains(&recherche) {
            result.insert(line, value_clone);
        }
    }
    return result;
}

#[cfg(test)]
mod testing {
    use std::fs::{self, metadata};
    use super::*;

    fn set_config() -> Config {
        let config = Config::new(&["".to_string(), "kouakou".to_string(), "src/derek.txt".to_string()]).expect("une erreur est survenue");
        config
    }

    #[test]
    fn test_ouverture() {
        let config = set_config();
        let result = ouverture(&config);
        let medata = fs::metadata(config.get_fichier()).unwrap();
        let file_type = medata.file_type();
        assert_eq!(file_type.is_file(), true);
    }

    #[test]
    fn test_lecture() {
        let mut file = File::open(set_config().get_fichier()).unwrap();
        let result = lecture(&mut file);
        assert_eq!(result.lines().count(), 5);
    }

    #[test]
    fn test_recherche() {
        let mut file = File::open("./src/derek.txt").unwrap();
        let result = BufReader::new(&mut file);
        let result = recherche(&set_config(), result);
        assert_eq!(result.len(), 5);
    }
}
