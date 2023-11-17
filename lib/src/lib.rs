use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub struct Config {
    pub recherche : String,
    pub fichier : String,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3  {
            return Err("il n'y a pas assez de paramètres.");
        }
        let recherche = &args[1];
        let fichier = &args[2];

        Ok(Config {
            recherche : recherche.to_string(),
            fichier : fichier.to_string(),
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
        };
    }

    return result;
}

#[cfg(test)]
mod tests {
    use std::fs::{self, metadata};
    use super::*;

    fn set_config() -> Config {
        let config = Config::new(&["".to_string(), "derek".to_string(), "src/derek.txt".to_string()]).expect("une erreur est survenue");
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
        assert_eq!(result.lines().count(), 3 );
    }
}
