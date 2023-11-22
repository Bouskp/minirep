extern crate lib;

use std::{env, process};
use std::error::Error;
use lib::{Config, lecture, ouverture, presentation, recherche, sensitive_fun};

fn main() {
    let args : Vec<String>= env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Erreur survenu lors de la lecture des paramètres: {}", err);
        process::exit(0);
    });

    if let Err(e) = run(&config) {
        println!("Erreur applicative : {}", e.to_string());
        process::exit(0)
    };
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut file = ouverture(config)?;
    let result;

    let buff_reader = lecture(&mut file);
    if config.is_sensitive {
         result = recherche(&config, buff_reader);

    } else {
         result = sensitive_fun(&config, buff_reader);
    }
    presentation(result, &config);
    Ok(())
}