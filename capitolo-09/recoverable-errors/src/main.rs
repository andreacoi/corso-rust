use std::fs::File;
// integro la libreria ErrorKind
// p. 9.2.1
use std::io::ErrorKind;

// Per gestire gli errori recuperabili RUST utilizza la enum Result<T, E> dove T ed E sono
// parametri di tipo generico. Solitamente è un modo per stabilire il comportamento del programma
// nei due stati previsti:
// - se OK comportati in un certo modo,
// - se ERR comportati in un altro.

enum Result<T, E> {
    Ok(T),
    Err(E),
}
fn main() {
    let greetings_file_result = File::open("file.txt");

    /*
    * è possibile gestire gli errori in questa maniera:
    * SEMPLICE, con soli due casi, oppure cercando di prevedere altre casistiche d'errore.
    *
    * Per cui questo codice:
    *  let gfile = match greetings_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Il file non esiste: {:?}", error)
        }
    }; */
    // può essere riscritto in questa maniera:
    // va integrata la libreria io ErrorKind che consente di gestire le casistiche d'errore.
    // p. 9.2.1 Match different errors
    let gfile = match greetings_file_result {
        // 1
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 2
            ErrorKind::NotFound => {
                // 3
                match File::create("file.txt") {
                    // 4
                    Ok(fc) => fc,
                    Err(e) => panic!("Errore nella creazione del file: {:?}", e),
                } // 4
            } // 3
            other_error => {
                panic!("Problema nell'apertura del file: {:?}", other_error)
            }
        }, //2
    }; // 1
}
