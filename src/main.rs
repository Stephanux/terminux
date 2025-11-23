use std::io;
use std::process::Command;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;

/**********************************************************************
 * fonction qui permet de lire les commandes au clavier
 **********************************************************************/
fn read_cmd() -> io::Result<String> {
    // type de Result un Result avec String si OK
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    // Supprime le retour à la ligne final
    Ok(buffer.trim().to_string()) // ajout de : .trim().to_string() pour supprimer le \n
}
/**********************************************************************
*   Structure pour récupérer les données du scénario stockées en JSON
***********************************************************************/
#[derive(Serialize, Deserialize)]
pub struct Question {
    libelle: String,
    commande: String,
    points: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Scenario {
    name: String,
    date: String,
    questions: Vec<Question>,
}

/**************************************************************************************
 * Fonction get_scenario permet de lire le fichier Json de configuraiton de l'exercice
 **************************************************************************************/
fn get_scenario() -> Result<Scenario, Box<dyn std::error::Error>> {
    let json_file_path = Path::new("./scenario.json");
    let file = std::fs::File::open(json_file_path)?;
    let _mon_scenario: Scenario = serde_json::from_reader(file)?;
    Ok(_mon_scenario)
}

/***********************************************************************
 * Fonction main principale point d'entrée du programme d'exercice Linux
************************************************************************/
fn main() {
    let mut _exercice: Scenario;
    let mut _compteur: usize = 0; // incrémenté après chaque question posée
    let mut _nb_questions: usize = 0; // Récupére la taille du vecteur qui stocke les questions
    let mut _points: u32 = 0;
    // todo : ici lire le fichier de configuration json du scenario de l'exercice et boucler tant qu'il y a des commandes
    let _mon_scenar = get_scenario();
    // on vérifie que la lecture c'est bien passée
    match _mon_scenar {
        Ok(v) => {
            _nb_questions = v.questions.len();
            println!("nom scenario : {}", v.name);
            println!("date exercice : {}", v.date);
            println!("nb questions : {:#?}", v.questions.len());
            // on boucle sur les questions
            while _compteur < _nb_questions {
                println!("{}", v.questions[_compteur].libelle);
                println!("Entrez la commande : ");
                let result = read_cmd();
                match result {
                    Ok(cmd) => {
                        let cmd_list: Vec<&str>= cmd.split(' ').collect();
                        println!("cmd_list : {:?}", cmd_list);
                        let mut command = Command::new(cmd_list[0]);
                        command.current_dir("/home/stephane/");
                        println!("dossier courant : {:?}", command.get_current_dir());
                        for _part in &cmd_list[1..] {
                            command.arg(_part);
                        }
                        let _result_cmd = command.output().expect("failed to execute process");
                        println!("resultat commande : {:?}", _result_cmd);
                        if cmd == v.questions[_compteur].commande {
                            _points += 1;  // on incrémente les points de l'exercice
                        }
                    }
                    Err(e) => println!("error input: {:?}", e),
                }
                _compteur += 1; // on passe à la question suivante
            }
        }
        Err(e) => println!("error input: {:?}", e),
    }

    println!("Points gagnés : {_points}")
}
