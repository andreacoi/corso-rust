/*
* Progetto Minigrep
* v.0.1
* Autore: Andrea Coi
*/

// Per poter eseguire il parse degli argomenti passati al programma tramite cli abbiamo bisogno
// della funzione args fornita dalla Rust stdlib.

use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    // la funzione collect crea un iteratore, una collezione di oggetti (di tipo String),
    // iterabile.
    // Per salvare gli args da cli è bene tenere a mente una cosa:
    // args[0] è il nome del programma in esecuzione. Quindi gli indici da 1 in poi sono quelli
    // destinati agli argomenti.
    // salvo gli argomenti nelle variabili
    // ATTENZIONE! Utilizzando build senza specificare il prefisso Config (la struct che poi fa da
    // appoggio alle funzioni associate) il sistema cercherà la funzione build in tutto lo scope
    // non trovandola. Questo perché build è una funzione associata PROPRIA DELLA STRUCT Config,
    // che abbiamo richiamato con il primo use.
    let config = Config::build(&args).unwrap_or_else(|err| {
        //println!("Problema nel parsing degli argomenti: {err}");
        // passo l'output degli errori dallo standard output all'error output
        eprintln!("Problema nel parsing degli argomenti: {err}");
        process::exit(1);
    });
    // stampo le due stringhe "di benvenuto" prima di iniziare la ricerca
    println!("Sto cercando {}", config.query);
    println!("nel file {}", config.file_path);
    // estraggo il ritorno di contents in un'altra funzione
    /* let contents = fs::read_to_string(config.file_path)
        .expect("dovrei poter leggere dal file... che succede?");
    println!("La ricerca verrà eseguita su questo testo: \n {contents}"); */
    // la funzione run non ritorna alcun valore ma viene utilizzata solo per stampare le variabili
    // e i risultati. Ciò significa che non possiamo utilizzare l'unwrap_or_else perché non abbiamo
    // valori di cui eseguire l'unwrap.
    // Ci viene in aiuto il costrutto if let che consente di gestire l'errore nel caso in cui si
    // presenti. Ragionamento in logica negativa.
    if let Err(e) = minigrep::run(config) {
        // passo l'output degli errori dallo standard output all'error output
        eprintln!("Errore nell'applicazione: {e}");
        process::exit(1);
    }
}
