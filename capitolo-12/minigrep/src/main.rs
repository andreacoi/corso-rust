/*
* Progetto Minigrep
* v.0.1
* Autore: Andrea Coi
*/

// Per poter eseguire il parse degli argomenti passati al programma tramite cli abbiamo bisogno
// della funzione args fornita dalla Rust stdlib.

use std::env;
// carico, sempre dalla standard library fs, che serve per gestire la lettura da file
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // la funzione collect crea un iteratore, una collezione di oggetti (di tipo String),
    // iterabile.
    // Per salvare gli args da cli è bene tenere a mente una cosa:
    // args[0] è il nome del programma in esecuzione. Quindi gli indici da 1 in poi sono quelli
    // destinati agli argomenti.
    // salvo gli argomenti nelle variabili
    let (query, file_path) = parse_config(&args);
    // stampo le due stringhe "di benvenuto" prima di iniziare la ricerca
    println!("Sto cercando {}", &query);
    println!("nel file {}", &file_path);
    let contents =
        fs::read_to_string(file_path).expect("dovrei poter leggere dal file... che succede?");
    println!("La ricerca verrà eseguita su questo testo: \n {contents}");
}

// al fine di manutenere in maniera più efficiente il parse degli argomenti vado ad estrarre gli
// stessi dal main in una funzione dedicata.

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
