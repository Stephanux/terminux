use std::io;
use std::process::Command;
use std::fs;
use serde_json::Result;

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

/**
 * Fonction get_scenario permet de lire le fichier Json de configuraiton de l'exercice
 */
fn get_scenario() {

}

/** 
 * Fonction main principale point d'entrée du programme
*/
fn main() {
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