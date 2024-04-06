/*
* Progetto Minigrep
* v.0.1
* Autore: Andrea Coi
*/

// Per poter eseguire il parse degli argomenti passati al programma tramite cli abbiamo bisogno
// della funzione args fornita dalla Rust stdlib.

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // la funzione collect crea un iteratore, una collezione di oggetti (di tipo String),
    // iterabile.
    // Per salvare gli args da cli è bene tenere a mente una cosa:
    // args[0] è il nome del programma in esecuzione. Quindi gli indici da 1 in poi sono quelli
    // destinati agli argomenti.
    // salvo gli argomenti nelle variabili
    let query = &args[1];
    let file_path = &args[2];
    // stampo le due stringhe "di benvenuto" prima di iniziare la ricerca
    println!("Sto cercando {}", query);
    println!("nel file {}", file_path);
}
