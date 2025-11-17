use std::io;
//use std::process::Command;
use std::path::Path;
use serde::Deserialize;
use serde::Serialize;


/**
 * fonction qui permet de lire les commandes au clavier
 */
fn read_cmd() -> io::Result<String> {  // type de Result un Result avec String si OK
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    // Supprime le retour à la ligne final
    Ok(buffer.trim().to_string()) // ajout de : .trim().to_string() pour supprimer le \n
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    libelle : String,
    commande: String,
    points: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Scenario {
    name: String,
    date: String,
    questions: Vec<Question>,
}
/**
 * Fonction get_scenario permet de lire le fichier Json de configuraiton de l'exercice
 */
fn get_scenario() -> Result<Scenario, Box<dyn std::error::Error>> {
    let json_file_path = Path::new("./scenario.json");
    let file = std::fs::File::open(json_file_path)?;
    let _mon_scenario: Scenario = serde_json::from_reader(file)?;
    Ok(_mon_scenario)
}

/** 
 * Fonction main principale point d'entrée du programme
*/
fn main() {
    let _mon_scenar = get_scenario();
    match _mon_scenar {
        Ok(v) => {
            println!("nom scenario : {}", v.name);
        },
        Err(e) => println!("error input: {:?}", e),
    }
    let mut _points : u32 = 0;
    println!("Entrez la commande: ");
    let result = read_cmd();
    // todo : ici lire le fichier de configuration json du scenario de l'exercice et boucler tant qu'il y a des commandes
    match result {
        Ok(v) => {
            println!("string input: {:?}", v);
            if v == "mkdir -p DirA/dirA1/dirA11" {
                _points +=1;
            }
        },
        Err(e) => println!("error input: {:?}", e),
    }
    println!("Points gagnés : {_points}")
}