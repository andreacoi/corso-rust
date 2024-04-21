// lib.rs
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
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
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Result<T,E> ritorna una clousure () "come T" e Box<dyn Error> come E.
    // Box<dyn Error> significa che quando la funzione ritorna un errore, lo fa ritornando un tipo
    // generico che implementa il trait Error.
    // Per questo motivo è necessario utilizzare all'inizio del file:
    // use std::error::Error;
    // che è l'interfaccia che ci consente di utilizzare il trait Error.
    let contents = fs::read_to_string(config.file_path)?;
    // con il ? ritorno un valore di tipo Errore generico invece di utilizzare expect che causa il
    // panic!
    // con l'espressione che segue specifico che in caso di Ok, ritorno una closure vuota. Questo
    // "costrutto" è idiomatico e signfica che in caso di Ok stiamo chiamando run() ESPRESSAMENTE
    // per i suoi effetti collaterali e per tali non necessitiamo di un valore di ritorno.
    // Tradotto: La funzione run() si occupa solo di stampare dei risultati e non ritorna alcun
    // valore.
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
/* al momento della stesura di questo test la funzione search ANCORA NON ESISTE, perciò questo test
* fallirà. Questo è proprio quanto sta alla base del test driven development.
* IL TDD - TEST DRIVEN DEVELOPMENT
* la logica alla base di questo tipo di sviluppo consiste nel fatto che lo sviluppo viene
* effettuato sulla base di quanto atteso dal risultato dei test.
* Avendo quindi stabilito, nel test "one_result" che il tipo di uguaglianza richiesta da assert_eq
* è un vec composto da una stringa, scriverò la funzione search in modo che possa essere in grado
* di eseguire il test INDIPENDEMENTE DAL SUO ESITO.
* la funzione search è così realizzata:
*/

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    // crea un vettore che verrà popolato con i risultati della ricerca.
    let mut results = Vec::new();
    // inizia a iterare nelle linee di content utilizzando il metodo lines()
    for line in content.lines() {
        // fa qualcosa SE line contiene query - salvataggio nel vettore e successivo ritorno.
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}
