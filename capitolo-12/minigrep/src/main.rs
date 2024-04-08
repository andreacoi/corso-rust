/*
* Progetto Minigrep
* v.0.1
* Autore: Andrea Coi
*/

// Per poter eseguire il parse degli argomenti passati al programma tramite cli abbiamo bisogno
// della funzione args fornita dalla Rust stdlib.

use core::panic;
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
    let config = Config::new(&args);
    // stampo le due stringhe "di benvenuto" prima di iniziare la ricerca
    println!("Sto cercando {}", config.query);
    println!("nel file {}", config.file_path);
    let contents = fs::read_to_string(config.file_path)
        .expect("dovrei poter leggere dal file... che succede?");
    println!("La ricerca verrà eseguita su questo testo: \n {contents}");
}

// al fine di manutenere in maniera più efficiente il parse degli argomenti vado ad estrarre gli
// stessi dal main in una funzione dedicata.
/*
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
 */
// parse config può essere riscritta per ritornare un elemento di tipo Config (che in realtà è una
// struct) per gestire gli args.
struct Config {
    query: String,
    file_path: String,
}
// implemento il costruttore --> invocando Config::new(&args) vado a inizializzare automaticamente
// la configurazione. Rimpiazza la funzione parse_config.

// Problema da risolvere
// Se gli args sono in un numero < 3 il programma andrà in panic, perché cercherà di accedere a un
// indice che non esiste (il nome del programma ha indice 0).

impl Config {
    fn new(args: &[String]) -> Config {
        //verifica che gli argomenti siano > 3 prima di eseguire il programma.
        if args.len() < 3 {
            panic!("devi dichiarare più argomenti");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
// ho scelto di clonare gli elementi di args perché è complicato gestire il lifetime delle
// variabili a partire dalla struct. Sostituita dal costruttore new nella impl Config.
/* fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
} */
