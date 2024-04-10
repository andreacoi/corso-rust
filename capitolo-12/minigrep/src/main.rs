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
use std::process;
// uso Error per gestire il Box Error nella funzione run()
use std::error::Error;
fn main() {
    let args: Vec<String> = env::args().collect();
    // la funzione collect crea un iteratore, una collezione di oggetti (di tipo String),
    // iterabile.
    // Per salvare gli args da cli è bene tenere a mente una cosa:
    // args[0] è il nome del programma in esecuzione. Quindi gli indici da 1 in poi sono quelli
    // destinati agli argomenti.
    // salvo gli argomenti nelle variabili
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema nel parsing degli argomenti: {err}");
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
    if let Err(e) = run(config) {
        println!("Errore nell'applicazione: {e}");
        process::exit(1);
    }
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
/* Per gestire gli errori in maniera pulita ricorro a Result<T, E>, enum particolare del capitolo 9,
* laddove T rappresenterà un'istanza di Config ed E un letterale stringa con lifetime static.
* Inoltre, il nome della funzione non sarà più new() perché solitamente si intendono con new delle
* funzioni che NON POSSONO FALLIRE. Riscrivo quindi la signature di new, che diventerà build.
*/
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        //verifica che gli argomenti siano > 3 prima di eseguire il programma.
        if args.len() < 3 {
            return Err("Devi dichiarare almeno tre argomenti...");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
// ho scelto di clonare gli elementi di args perché è complicato gestire il lifetime delle
// variabili a partire dalla struct. Sostituita dal costruttore new nella impl Config.
/* fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
} */
// gestiamo gli errori derivanti dalla funzione -- SEMPRE CON IL SOLITO RESULT<T,E>
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Result<T,E> ritorna una clousure () "come T" e Box<dyn Error> come E.
    // Box<dyn Error> significa che quando la funzione ritorna un errore, lo fa ritornando un tipo
    // generico che implementa il trait Error.
    // Per questo motivo è necessario utilizzare all'inizio del file:
    // use std::error::Error;
    // che è l'interfaccia che ci consente di utilizzare il trait Error.
    let contents = fs::read_to_string(config.file_path)?;
    // con il ? ritorno un valore di tipo Errore generico invece di utilizzare expect che causa il
    // panic!

    println!("La ricerca verrà eseguita su questo testo: \n {contents}");
    // con l'espressione che segue specifico che in caso di Ok, ritorno una closure vuota. Questo
    // "costrutto" è idiomatico e signfica che in caso di Ok stiamo chiamando run() ESPRESSAMENTE
    // per i suoi effetti collaterali e per tali non necessitiamo di un valore di ritorno.
    // Tradotto: La funzione run() si occupa solo di stampare dei risultati e non ritorna alcun
    // valore.
    Ok(())
    // TODO: gestire errori riportati da warn.
}
