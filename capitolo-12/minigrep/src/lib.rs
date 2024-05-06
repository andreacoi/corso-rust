// lib.rs
use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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
    // modifico l'argomento di build, adesso args è un generic che implementa il trait Iterator.
    // questa sintassi significa che:
    // ARGS PUÒ ESSERE QUALSIASI TIPO CHE PERÒ(!!!) IMPLEMENTI UN TIPO ITERATORE E RITORNI
    // NECESSARIAMENTE UN OGGETTO STRINGA.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // rimuovo il check del numero degli argomenti perché lo gestirò con delle Option.
        // utilizzo args.next() perché l'argomento "con indice 0" è il nome dell'applicazione.
        args.next();
        // Riscrittura con nozioni dal capitolo 13
        // al momento della stesura di questo codice abbiamo optato per l'utilizzo di clone, anche
        // se DELIBERATAMENTE non ottimizzato. Abbiamo scelto di utilizzare il metodo clone, perché
        // abbiamo una slice di Stringhe, ma questa funzione, la funzione build, non ha il
        // "possesso" di args. Abbiamo quindi scelto, per evitare problemi con il borrow checker di
        // utilizzare il metodo clone. Così la funzione build ha il possesso di query e file_path
        // che sono entrambi delle clonazioni rispettivamente di args[1] e di args[2].
        //
        //query e file path li ricavo tramite delle Option, utilizzando il costrutto match.
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Non hai specificato una stringa da cercare."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Non riesco a trovare il file che cerchi nel percorso specificato.")
            }
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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
    // Dopo aver verificato che tutti i test sono andati a buon fine posso integrare la funzione
    // search nella funzione run().
    // for line in search(&config.query, &contents) {
    //     println!("{line}");
    // }

    // popola let con condizione if/else. se ignore_case = 1 (SETTATO COME ENV VARIABLE) associa a
    // result il valore di ritorno della funzione search_case_insensitive, altrimenti quello della
    // funzione search.
    //
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    // stabilito il funzionamento della funzione search così com'è, andiamo ad aggiungere una nuova
    // feature al programma,.utiòizzando le variabili d'ambiente. In particolare possiamo scrivere
    // un tipo di ricerca che tenga conto di maiuscole e minuscole (case sensitive) e uno che NON
    // tenga presente di tale differenza (case insensitive). Per scrivere tali funzioni seguiamo
    // sempre lo stesso procedimento, il TDD. Quindi vado a copiare le funzioni in test e lavoro su
    // quelle.
    // Secondo quando stabilito dalla modalità TDD, bisogna impostare PRIMA i test per farli
    // fallire e successivamente correggerne il funzionamento per arrivare al funzionamento
    // corretto.
    //
    #[test]
    fn case_insensitve() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
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
    /*
    * Vecchia versione del metodo search.
    * Anche questo può essere riscritto utilizzando le closure e gli iteratori.
    * let mut results = Vec::new();
        // inizia a iterare nelle linee di content utilizzando il metodo lines()
        for line in content.lines() {
            // fa qualcosa SE line contiene query - salvataggio nel vettore e successivo ritorno.
            if line.contains(query) {
                results.push(line)
            }
        }
        results
    }
    * Questo metodo può essere ridotto ad usa sola riga, partendo da content.
    */
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    // SPIEGAZIONE DI QUESTA SERIE DI METODI CONCATENATI
    // Innanzitutto, tutti i metodi insistono su content. Quindi tutte le operazioni vengono
    // effettuate su content. Questa serie di metodi ritorna un Vettore di slice, che è quanto
    // richiesto dalla signature del metodo search. Il ritorno viene effettuato omettendo il punto
    // e virgola alla fine dell'espressione. Il metodo collect, inoltre, garantisce che "la
    // variabile" ritornata sia effettivamente una collezione di elementi.
    // Questo per quanto concerne, la cosa generale.
    // Scendendo nei particolari, analizzo i metodi:
    // - lines() - è una funzione che consente di ottenere un iteratore, che itera sulle singole
    // linee di una stringa, trattandole come slice.
    // - filter() - colleziona dei risultati solo se soddisfano la condizione specificata tra
    // parentesi.
    // - |line| è una closure booleana che restitusce il risultato della funzione affianco. In
    // particolare se line.contains(query) restituisce valore vero, line diventa la slice che
    // soddisfa la condizione. Non appena viene popolata "line" filter agisce tenendo conto della
    // linea corrente.
    // - collect() - trasforma le slice in un vettore di slice, che è il tipo di ritorno richiesto
    // dalla signature della funzione.
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    // eseguo lo shadowing di query perché non avrò più bisogno del suo valore precedente.
    // N.B. il metodo lowercase potrebbe essere impreciso nella conversione di un Unicode.
    let query = query.to_lowercase();
    // così come per il test precedente inizializzo uno vettore vuoto.
    let mut results = Vec::new();
    // vado a ciclare le linee del contenuto, utilizzando il metodo lines()
    for line in content.lines() {
        // dopo aver convertito query in minuscolo, faccio lo stesso con line, prima di verificare
        // se QUELLA linea contiene la query ricercata.
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
