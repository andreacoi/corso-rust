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
    /*     let gfile = match greetings_file_result {
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
    }; // 1 */
    // 9.2.1.1 Alternative all'utilizzo di match con Result<T, E>
    // È possibile riscrivere il codice sopra utilizzando le closure per ottenere un codice più
    // pulito e del tutto equivalente.

    let gfile = File::open("file.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("file.txt").unwrap_or_else(|error| {
                panic!("Errore nella creazione del file: {:?}", error);
            })
        } else {
            panic!("Problema generico nell'apertura del file: {:?}", error);
        }
    });
    // Le closure semplificano di molto la sintassi necessaria e vengono trattate nella maniera più
    // specifica nel capitolo 13.
    //
    // 9.2.1.2 Scorciatoia per Panic sull'error: expect e unwrap.
    // Oltre alle closure e a match sulle enum abbiamo un altro modo ancora più compatto per
    // gestire i panic.
    // Il modo è: utilizzare expect e unwrap. Expect consente di specificare un messaggio
    // personalizzato nel caso di errore.
    // Expect infatti ritorna T in caso di successo ed E in caso di errore.
    // Unwrap è identico, a parte il fatto che non è possibile specificare un messaggio per il
    // panic.
    let new_file = File::open("gino.txt").expect("Il file non esiste.");
    // Di solito gli sviluppatori Rust tendono ad utilizzare expect.
}
